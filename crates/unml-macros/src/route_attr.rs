//! Parser for the `#[route(...)]` attribute.

use syn::spanned::Spanned;
use syn::{Attribute, Ident, LitStr, Result, Token};

pub struct RouteAttr {
    pub id: LitStr,
    pub label: LitStr,
    pub icon: Option<Ident>,
    pub is_plain: bool,
}

impl RouteAttr {
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        for attr in attrs {
            if attr.path().is_ident("route") {
                return Self::parse(attr);
            }
        }

        Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Missing #[route(...)] attribute",
        ))
    }

    fn parse(attr: &Attribute) -> Result<Self> {
        let mut id: Option<LitStr> = None;
        let mut label: Option<LitStr> = None;
        let mut icon: Option<Ident> = None;
        let mut is_plain = false;

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("id") {
                meta.input.parse::<Token![=]>()?;
                id = Some(meta.input.parse()?);
            } else if meta.path.is_ident("label") {
                meta.input.parse::<Token![=]>()?;
                label = Some(meta.input.parse()?);
            } else if meta.path.is_ident("icon") {
                meta.input.parse::<Token![=]>()?;
                icon = Some(meta.input.parse()?);
            } else if meta.path.is_ident("plain") {
                is_plain = true;
            } else {
                return Err(syn::Error::new(
                    meta.path.span(),
                    format!("Unknown route attribute: {:?}", meta.path.get_ident()),
                ));
            }

            Ok(())
        })?;

        let id = id.ok_or_else(|| {
            syn::Error::new(
                attr.bracket_token.span.join(),
                "Missing `id` in #[route(...)]",
            )
        })?;

        let label = label.ok_or_else(|| {
            syn::Error::new(
                attr.bracket_token.span.join(),
                "Missing `label` in #[route(...)]",
            )
        })?;

        Ok(RouteAttr {
            id,
            label,
            icon,
            is_plain,
        })
    }
}
