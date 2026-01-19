//! Implementation of the `define_sidebar!` macro.

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitStr, Result, Token, braced};

/// A single sidebar item: `route_id => "label"`
///
/// The route_id should match a child route defined in `define_app_routes!`.
struct SidebarItem {
    route_id: Ident,
    label: LitStr,
}

impl Parse for SidebarItem {
    fn parse(input: ParseStream) -> Result<Self> {
        let route_id: Ident = input.parse()?;
        input.parse::<Token![=>]>()?;
        let label: LitStr = input.parse()?;

        Ok(SidebarItem { route_id, label })
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
///
/// Generates:
/// - `SIDEBAR: Option<&'static SidebarContent>` - sidebar content for the page layout
/// - `VARIANT: Option<SidebarVariant>` - sidebar variant (Filter or Navigation)
/// - `DEFAULT_ID: Option<&'static str>` - default child route ID
///
/// Does NOT generate `Selection` enum - pages should use the selection parameter directly.
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

    // Collect all items to find the first one (default)
    let first_item = def
        .sections
        .iter()
        .flat_map(|s| s.items.iter())
        .next();

    let first_item = match first_item {
        Some(item) => item,
        None => {
            return syn::Error::new(
                proc_macro2::Span::call_site(),
                "Sidebar must have at least one item",
            )
            .to_compile_error();
        }
    };

    let default_id = first_item.route_id.to_string();

    // Generate sidebar sections
    let section_tokens: Vec<_> = def
        .sections
        .iter()
        .map(|section| {
            let items: Vec<_> = section
                .items
                .iter()
                .map(|item| {
                    let id = item.route_id.to_string();
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
        pub const SIDEBAR: Option<&'static crate::components::sidebar::SidebarContent> = Some(&crate::components::sidebar::SidebarContent::new(&[
            #(#section_tokens),*
        ]));

        pub const VARIANT: Option<crate::components::sidebar::SidebarVariant> = Some(crate::components::sidebar::SidebarVariant::#variant_ident);

        pub const DEFAULT_ID: Option<&'static str> = Some(#default_id);
    }
}
