//! Parser for the `#[route(...)]` attribute.
//!
//! This module handles parsing of route attributes like:
//! ```ignore
//! #[route(path = "/versions", label = "nav.versions", icon = Folder)]
//! #[route(path = "/", label = "nav.home", icon = LayoutDashboard, home)]
//! ```

use syn::spanned::Spanned;
use syn::{Attribute, Ident, LitStr, Result, Token};

pub struct RouteAttr {
    pub path: LitStr,
    pub label: LitStr,
    pub icon: Option<Ident>,
    pub is_home: bool,
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
            "Missing #[route(...)] attribute. Expected: #[route(path = \"...\", label = \"...\", icon = IconName)]",
        ))
    }

    fn parse(attr: &Attribute) -> Result<Self> {
        let mut path: Option<LitStr> = None;
        let mut label: Option<LitStr> = None;
        let mut icon: Option<Ident> = None;
        let mut is_home = false;

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("path") {
                meta.input.parse::<Token![=]>()?;
                path = Some(meta.input.parse()?);
            } else if meta.path.is_ident("label") {
                meta.input.parse::<Token![=]>()?;
                label = Some(meta.input.parse()?);
            } else if meta.path.is_ident("icon") {
                meta.input.parse::<Token![=]>()?;
                icon = Some(meta.input.parse()?);
            } else if meta.path.is_ident("home") {
                is_home = true;
            } else {
                return Err(syn::Error::new(
                    meta.path.span(),
                    format!("Unknown route attribute: {:?}", meta.path.get_ident()),
                ));
            }

            Ok(())
        })?;

        let path = path.ok_or_else(|| {
            syn::Error::new(
                attr.bracket_token.span.join(),
                "Missing `path` in #[route(...)]",
            )
        })?;

        let label = label.ok_or_else(|| {
            syn::Error::new(
                attr.bracket_token.span.join(),
                "Missing `label` in #[route(...)]",
            )
        })?;

        Ok(RouteAttr {
            path,
            label,
            icon,
            is_home,
        })
    }

    pub fn id(&self) -> String {
        let path = self.path.value();
        if let Some(pos) = path.rfind('/') {
            path[pos + 1..].to_string()
        } else {
            path
        }
    }
}
