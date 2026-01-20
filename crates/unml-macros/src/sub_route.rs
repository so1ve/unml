//! Implementation of the `#[derive(SubRoute)]` macro.
//!
//! This module provides a simplified derive macro for child/sub pages
//! that only require an ID. Unlike `PageRoute`, sub-routes don't need
//! paths, labels, or icons since they are rendered within a parent page.
//!
//! # Usage
//!
//! ```ignore
//! #[derive(SubRoute)]
//! #[subroute(id = "java")]
//! pub struct JavaSettingsPage;
//! ```
//!
//! # Attributes
//!
//! - `id`: The route identifier used to match child routes (required)

use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Attribute, DeriveInput, LitStr, Result, Token};

struct SubRouteAttr {
    id: LitStr,
}

impl SubRouteAttr {
    fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        for attr in attrs {
            if attr.path().is_ident("subroute") {
                return Self::parse(attr);
            }
        }

        Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Missing #[subroute(...)] attribute. Expected: #[subroute(id = \"...\")]",
        ))
    }

    fn parse(attr: &Attribute) -> Result<Self> {
        let mut id: Option<LitStr> = None;

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("id") {
                meta.input.parse::<Token![=]>()?;
                id = Some(meta.input.parse()?);
            } else {
                return Err(syn::Error::new(
                    meta.path.span(),
                    format!("Unknown subroute attribute: {:?}", meta.path.get_ident()),
                ));
            }

            Ok(())
        })?;

        let id = id.ok_or_else(|| {
            syn::Error::new(
                attr.bracket_token.span.join(),
                "Missing `id` in #[subroute(...)]",
            )
        })?;

        Ok(SubRouteAttr { id })
    }
}

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;
    let attr = SubRouteAttr::from_attrs(&input.attrs)?;
    let id = &attr.id;

    let expanded = quote! {
        impl crate::routing::SubRoute for #name {
            const ID: &'static str = #id;

            fn render(window: &mut gpui::Window, cx: &mut gpui::App) -> gpui::AnyElement {
                gpui::IntoElement::into_any_element(Self::view(window, cx))
            }
        }
    };

    Ok(expanded)
}
