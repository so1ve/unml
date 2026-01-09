//! Implementation of the `define_sidebar!` macro.

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitStr, Result, Token, braced};

/// Convert PascalCase to snake_case
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}

/// A single sidebar item: `Variant => "label"`
struct SidebarItem {
    variant: Ident,
    label: LitStr,
}

impl Parse for SidebarItem {
    fn parse(input: ParseStream) -> Result<Self> {
        let variant: Ident = input.parse()?;
        input.parse::<Token![=>]>()?;
        let label: LitStr = input.parse()?;

        Ok(SidebarItem { variant, label })
    }
}

/// A sidebar section: `section "title" { items... }` or `section { items... }`
struct SidebarSection {
    title: Option<LitStr>,
    items: Vec<SidebarItem>,
}

impl Parse for SidebarSection {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse "section" keyword
        let ident: Ident = input.parse()?;
        if ident != "section" {
            return Err(syn::Error::new(ident.span(), "expected `section`"));
        }

        // Optional title
        let title = if input.peek(LitStr) {
            Some(input.parse()?)
        } else {
            None
        };

        // Parse items in braces
        let content;
        braced!(content in input);

        let mut items = Vec::new();
        while !content.is_empty() {
            items.push(content.parse()?);
            // Allow trailing comma
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }

        Ok(SidebarSection { title, items })
    }
}

/// Sidebar variant: Filter or Navigation
struct SidebarVariant {
    variant: Ident,
}

impl Parse for SidebarVariant {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;
        if ident != "variant" {
            return Err(syn::Error::new(ident.span(), "expected `variant`"));
        }
        input.parse::<Token![:]>()?;
        let variant: Ident = input.parse()?;

        // Validate variant name
        let variant_str = variant.to_string();
        if variant_str != "Filter" && variant_str != "Navigation" {
            return Err(syn::Error::new(
                variant.span(),
                "expected `Filter` or `Navigation`",
            ));
        }

        Ok(SidebarVariant { variant })
    }
}

/// The full sidebar definition
struct SidebarDef {
    variant: Option<SidebarVariant>,
    sections: Vec<SidebarSection>,
}

impl Parse for SidebarDef {
    fn parse(input: ParseStream) -> Result<Self> {
        // Empty definition
        if input.is_empty() {
            return Ok(SidebarDef {
                variant: None,
                sections: Vec::new(),
            });
        }

        // Parse variant
        let variant = Some(input.parse()?);

        // Allow comma after variant
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        // Parse sections
        let mut sections = Vec::new();
        while !input.is_empty() {
            sections.push(input.parse()?);
        }

        Ok(SidebarDef { variant, sections })
    }
}

/// Core implementation of the define_sidebar macro.
pub fn define(input: TokenStream) -> TokenStream {
    let def: SidebarDef = match syn::parse2(input) {
        Ok(def) => def,
        Err(err) => return err.to_compile_error(),
    };

    // Empty definition - generate None values
    if def.variant.is_none() {
        return quote! {
            pub const SIDEBAR: Option<&'static crate::components::sidebar::SidebarContent> = None;
            pub const VARIANT: Option<crate::components::sidebar::SidebarVariant> = None;
            pub const DEFAULT_ID: Option<&'static str> = None;
        };
    }

    let variant_ident = &def.variant.as_ref().unwrap().variant;

    // Collect all items with their info
    struct ItemInfo {
        variant: Ident,
        id: String,
    }

    let mut all_items: Vec<ItemInfo> = Vec::new();

    for section in &def.sections {
        for item in &section.items {
            let id = to_snake_case(&item.variant.to_string());
            all_items.push(ItemInfo {
                variant: item.variant.clone(),
                id,
            });
        }
    }

    // Use first item as default
    let first_item = match all_items.first() {
        Some(item) => item,
        None => {
            return syn::Error::new(
                proc_macro2::Span::call_site(),
                "Sidebar must have at least one item",
            )
            .to_compile_error();
        }
    };

    let default_variant = &first_item.variant;
    let default_id = &first_item.id;

    // Generate Selection enum
    let enum_variants: Vec<_> = all_items.iter().map(|item| &item.variant).collect();

    // Generate id() match arms
    let id_arms: Vec<_> = all_items
        .iter()
        .map(|item| {
            let variant = &item.variant;
            let id = &item.id;
            quote! { Self::#variant => #id }
        })
        .collect();

    // Generate from_id() match arms
    let from_id_arms: Vec<_> = all_items
        .iter()
        .map(|item| {
            let variant = &item.variant;
            let id = &item.id;
            quote! { #id => Self::#variant }
        })
        .collect();

    // Generate sidebar sections
    let section_tokens: Vec<_> = def
        .sections
        .iter()
        .map(|section| {
            let items: Vec<_> = section
                .items
                .iter()
                .map(|item| {
                    let id = to_snake_case(&item.variant.to_string());
                    let label = &item.label;
                    quote! {
                        crate::components::sidebar::SidebarItem::new(#id, #label)
                    }
                })
                .collect();

            match &section.title {
                Some(title) => quote! {
                    crate::components::sidebar::SidebarSection::new(&[#(#items),*]).with_title(#title)
                },
                None => quote! {
                    crate::components::sidebar::SidebarSection::new(&[#(#items),*])
                },
            }
        })
        .collect();

    quote! {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum Selection {
            #(#enum_variants),*
        }

        impl Selection {
            /// Returns the string ID for this selection.
            #[inline]
            pub const fn id(&self) -> &'static str {
                match self {
                    #(#id_arms),*
                }
            }

            /// Creates a selection from a string ID.
            #[inline]
            pub fn from_id(id: &str) -> Self {
                match id {
                    #(#from_id_arms,)*
                    _ => unreachable!(),
                }
            }
        }

        impl ::core::default::Default for Selection {
            #[inline]
            fn default() -> Self {
                Self::#default_variant
            }
        }

        pub const SIDEBAR: Option<&'static crate::components::sidebar::SidebarContent> = Some(&crate::components::sidebar::SidebarContent::new(&[
            #(#section_tokens),*
        ]));

        pub const VARIANT: Option<crate::components::sidebar::SidebarVariant> = Some(crate::components::sidebar::SidebarVariant::#variant_ident);

        pub const DEFAULT_ID: Option<&'static str> = Some(#default_id);
    }
}
