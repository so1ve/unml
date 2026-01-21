//! Parser for the `#[sidebar(...)]` attribute.
//!
//! This module handles parsing of sidebar attributes like:
//! ```ignore
//! #[sidebar(
//!     variant = Filter,
//!     section "versions.filter" {
//!         Release => "versions.release",
//!         Snapshot => "versions.snapshot",
//!     }
//! )]
//! ```
//!
//! The default selected item is always the first item in the first section.

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Ident, LitStr, Result, Token, braced};

/// A single sidebar item: `id => "label"`
pub struct SidebarItemDef {
    pub id: Ident,
    pub label: LitStr,
}

impl Parse for SidebarItemDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let id: Ident = input.parse()?;
        input.parse::<Token![=>]>()?;
        let label: LitStr = input.parse()?;

        Ok(SidebarItemDef { id, label })
    }
}

/// A sidebar section: `section "title" { items... }` or `section { items... }`
pub struct SidebarSectionDef {
    pub title: Option<LitStr>,
    pub items: Vec<SidebarItemDef>,
}

impl Parse for SidebarSectionDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;
        if ident != "section" {
            return Err(syn::Error::new(ident.span(), "expected `section`"));
        }

        let title = if input.peek(LitStr) {
            Some(input.parse()?)
        } else {
            None
        };

        let content;
        braced!(content in input);

        let mut items = Vec::new();
        while !content.is_empty() {
            items.push(content.parse()?);
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }

        Ok(SidebarSectionDef { title, items })
    }
}

/// Parsed sidebar attribute data.
pub struct SidebarAttr {
    pub variant: Ident,
    pub sections: Vec<SidebarSectionDef>,
}

impl SidebarAttr {
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Option<Self>> {
        for attr in attrs {
            if attr.path().is_ident("sidebar") {
                return Self::parse(attr).map(Some);
            }
        }

        Ok(None)
    }

    fn parse(attr: &Attribute) -> Result<Self> {
        let tokens = attr.meta.require_list()?.tokens.clone();

        syn::parse2(tokens)
    }

    pub fn default_id(&self) -> String {
        self.sections[0].items[0].id.to_string()
    }

    pub fn generate_sections(&self) -> TokenStream {
        let section_tokens: Vec<_> = self
            .sections
            .iter()
            .map(|section| {
                let items: Vec<_> = section
                    .items
                    .iter()
                    .map(|item| {
                        let id = item.id.to_string();
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

        quote! { #(#section_tokens),* }
    }
}

impl Parse for SidebarAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut variant: Option<Ident> = None;
        let mut sections = Vec::new();

        while !input.is_empty() {
            let lookahead = input.lookahead1();

            if lookahead.peek(Ident) {
                let ident: Ident = input.parse()?;

                if ident == "variant" {
                    input.parse::<Token![=]>()?;
                    variant = Some(input.parse()?);
                } else if ident == "section" {
                    let title = if input.peek(LitStr) {
                        Some(input.parse()?)
                    } else {
                        None
                    };

                    let content;
                    braced!(content in input);

                    let mut items = Vec::new();
                    while !content.is_empty() {
                        items.push(content.parse()?);
                        if content.peek(Token![,]) {
                            content.parse::<Token![,]>()?;
                        }
                    }

                    sections.push(SidebarSectionDef { title, items });
                } else {
                    return Err(syn::Error::new(
                        ident.span(),
                        format!("Unknown sidebar attribute: {}", ident),
                    ));
                }

                if input.peek(Token![,]) {
                    input.parse::<Token![,]>()?;
                }
            } else {
                return Err(lookahead.error());
            }
        }

        let variant = variant.ok_or_else(|| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                "Missing `variant` in #[sidebar(...)]",
            )
        })?;

        if sections.is_empty() {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "At least one section is required in #[sidebar(...)]",
            ));
        }

        if sections[0].items.is_empty() {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "First section must have at least one item",
            ));
        }

        Ok(SidebarAttr { variant, sections })
    }
}
