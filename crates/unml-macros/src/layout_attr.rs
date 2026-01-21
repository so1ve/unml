//! Parser for the `#[layout(...)]` attribute.

use syn::spanned::Spanned;
use syn::{Attribute, LitStr, Result, Token};

pub struct LayoutAttr {
    pub title: Option<LitStr>,
}

impl LayoutAttr {
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Option<Self>> {
        for attr in attrs {
            if attr.path().is_ident("layout") {
                return Self::parse(attr).map(Some);
            }
        }

        Ok(None)
    }

    fn parse(attr: &Attribute) -> Result<Self> {
        let mut title: Option<LitStr> = None;

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("title") {
                meta.input.parse::<Token![=]>()?;
                title = Some(meta.input.parse()?);
            } else {
                return Err(syn::Error::new(
                    meta.path.span(),
                    format!("Unknown layout attribute: {:?}", meta.path.get_ident()),
                ));
            }

            Ok(())
        })?;

        Ok(LayoutAttr { title })
    }
}
