//! Implementation of the `#[derive(PageRoute)]` macro.
//!
//! This module provides the derive macro that implements the `PageRoute` trait
//! for page structs.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, Result};

use crate::route_attr::RouteAttr;
use crate::sidebar_attr::SidebarAttr;

fn parse_children_attr(attrs: &[syn::Attribute]) -> Result<Vec<Ident>> {
    for attr in attrs {
        if attr.path().is_ident("children") {
            let mut children = Vec::new();
            attr.parse_nested_meta(|meta| {
                if let Some(ident) = meta.path.get_ident() {
                    children.push(ident.clone());
                }

                Ok(())
            })?;

            return Ok(children);
        }
    }

    Ok(Vec::new())
}

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;
    let route_attr = RouteAttr::from_attrs(&input.attrs)?;
    let sidebar_attr = SidebarAttr::from_attrs(&input.attrs)?;
    let children = parse_children_attr(&input.attrs)?;

    let path = &route_attr.path;
    let id = route_attr.id();
    let label = &route_attr.label;
    let is_home = route_attr.is_home;

    let icon_const = match &route_attr.icon {
        Some(icon) => quote! {
            const ICON: Option<gpui_component::IconName> = Some(gpui_component::IconName::#icon);
        },
        None => quote! {},
    };

    let (sidebar_const, variant_const, default_id_const) = if let Some(ref sidebar) = sidebar_attr {
        let sections = sidebar.generate_sections();
        let variant = &sidebar.variant;
        let default_id = sidebar.default_id();

        (
            quote! {
                const SIDEBAR: Option<&'static crate::components::sidebar::SidebarContent> = Some(&crate::components::sidebar::SidebarContent::new(&[
                    #sections
                ]));
            },
            quote! {
                const SIDEBAR_VARIANT: Option<crate::components::sidebar::SidebarVariant> = Some(crate::components::sidebar::SidebarVariant::#variant);
            },
            quote! {
                const DEFAULT_ID: &'static str = #default_id;
            },
        )
    } else {
        (quote! {}, quote! {}, quote! {})
    };

    let children_type = if children.is_empty() {
        quote! { type Children = (); }
    } else {
        let child_types: Vec<_> = children.iter().collect();
        quote! { type Children = (#(#child_types,)*); }
    };

    let children_impl = if !children.is_empty() {
        let child_match_arms: Vec<_> = children
            .iter()
            .map(|child| {
                quote! {
                    id if id == <#child as crate::routing::PageRoute>::ID => {
                        Some(<#child as crate::routing::PageRoute>::render(window, cx))
                    }
                }
            })
            .collect();

        quote! {
            impl crate::routing::ChildRoutes for (#(#children,)*) {
                fn render(id: &str, window: &mut gpui::Window, cx: &mut gpui::App) -> Option<gpui::AnyElement> {
                    match id {
                        #(#child_match_arms)*
                        _ => None,
                    }
                }

                fn ids() -> &'static [&'static str] {
                    const IDS: &[&str] = &[#(
                        <#children as crate::routing::PageRoute>::ID
                    ),*];
                    IDS
                }
            }
        }
    } else {
        quote! {}
    };

    let expanded = quote! {
        impl crate::routing::PageRoute for #name {
            const PATH: &'static str = #path;
            const ID: &'static str = #id;
            const LABEL: &'static str = #label;
            const IS_HOME: bool = #is_home;

            #icon_const
            #sidebar_const
            #variant_const
            #default_id_const

            #children_type

            fn render(window: &mut gpui::Window, cx: &mut gpui::App) -> gpui::AnyElement {
                gpui::IntoElement::into_any_element(Self::view(window, cx))
            }
        }

        #children_impl
    };

    Ok(expanded)
}
