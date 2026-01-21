//! Implementation of the `#[derive(SubRoute)]` macro.

use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Attribute, DeriveInput, LitStr, Result, Token};

use crate::layout_attr::LayoutAttr;

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
            "Missing #[subroute(...)] attribute",
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
    let layout_attr = LayoutAttr::from_attrs(&input.attrs)?;
    let id = &attr.id;

    let title_const = if let Some(layout) = layout_attr {
        if let Some(title) = layout.title {
            quote! {
                const TITLE: Option<&'static str> = Some(#title);
            }
        } else {
            quote! {}
        }
    } else {
        quote! {}
    };

    let expanded = quote! {
        impl crate::routing::SubRoute for #name {
            const ID: &'static str = #id;
            #title_const

            fn render(window: &mut gpui::Window, cx: &mut gpui::App) -> gpui::AnyElement {
                let view = <Self as crate::routing::PageView>::view(window, cx);
                gpui::IntoElement::into_any_element(
                    crate::components::layout::PageContent::new(Self::TITLE, view.into_any_element())
                )
            }
        }
    };

    Ok(expanded)
}
