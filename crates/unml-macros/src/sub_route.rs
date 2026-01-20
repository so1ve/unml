//! Implementation of the `#[derive(SubRoute)]` macro.
//!
//! This module provides a simplified derive macro for child/sub pages
//! that only require a path. Unlike `PageRoute`, sub-routes don't need
//! labels or icons since they are not shown in navigation.

use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Attribute, DeriveInput, LitStr, Result, Token};

struct SubRouteAttr {
    path: LitStr,
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
            "Missing #[subroute(...)] attribute. Expected: #[subroute(path = \"...\")]",
        ))
    }

    fn parse(attr: &Attribute) -> Result<Self> {
        let mut path: Option<LitStr> = None;

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("path") {
                meta.input.parse::<Token![=]>()?;
                path = Some(meta.input.parse()?);
            } else {
                return Err(syn::Error::new(
                    meta.path.span(),
                    format!("Unknown subroute attribute: {:?}", meta.path.get_ident()),
                ));
            }

            Ok(())
        })?;

        let path = path.ok_or_else(|| {
            syn::Error::new(
                attr.bracket_token.span.join(),
                "Missing `path` in #[subroute(...)]",
            )
        })?;

        Ok(SubRouteAttr { path })
    }
}

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;
    let attr = SubRouteAttr::from_attrs(&input.attrs)?;
    let path = &attr.path;

    let expanded = quote! {
        impl crate::routing::PageRoute for #name {
            const PATH: &'static str = #path;
            const LABEL: &'static str = "";

            type Children = ();

            fn render(window: &mut gpui::Window, cx: &mut gpui::App) -> gpui::AnyElement {
                gpui::IntoElement::into_any_element(Self::view(window, cx))
            }
        }
    };

    Ok(expanded)
}
