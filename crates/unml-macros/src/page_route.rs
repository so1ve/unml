//! Implementation of the `#[derive(PageRoute)]` macro.

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

    let id = &route_attr.id;
    let label = &route_attr.label;
    let is_plain = route_attr.is_plain;

    let icon_expr = match &route_attr.icon {
        Some(icon) => quote! { Some(gpui_component::IconName::#icon) },
        None => quote! { None },
    };

    if is_plain {
        let expanded = quote! {
            impl crate::routing::Routable for #name {
                fn route() -> crate::routing::PageRoute {
                    crate::routing::PageRoute::Plain(crate::routing::PlainRouteDef {
                        id: #id,
                        label: #label,
                        icon: #icon_expr,
                        render: |w, cx| {
                            gpui::IntoElement::into_any_element(
                                <Self as crate::routing::PageView>::view(w, cx),
                            )
                        },
                    })
                }
            }
        };

        Ok(expanded)
    } else {
        if children.is_empty() {
            return Err(syn::Error::new(
                input.ident.span(),
                "Sidebar routes require at least one child route. Add #[children(...)] attribute.",
            ));
        }

        let sidebar = sidebar_attr.ok_or_else(|| {
            syn::Error::new(
                input.ident.span(),
                "Sidebar routes require #[sidebar(...)] attribute.",
            )
        })?;

        let sections = sidebar.generate_sections();
        let variant = &sidebar.variant;
        let default_id = sidebar.default_id();

        let first_child = &children[0];
        let child_match_arms: Vec<_> = children
            .iter()
            .map(|child| {
                quote! {
                    id if id == <#child as crate::routing::SubRoute>::ID => {
                        <#child as crate::routing::SubRoute>::render(window, cx)
                    }
                }
            })
            .collect();

        let children_impl = quote! {
            impl crate::routing::ChildRoutes for (#(#children,)*) {
                fn render(id: &str, window: &mut gpui::Window, cx: &mut gpui::App) -> gpui::AnyElement {
                    match id {
                        #(#child_match_arms)*
                        _ => <#first_child as crate::routing::SubRoute>::render(window, cx),
                    }
                }
            }
        };

        let children_tuple = quote! { (#(#children,)*) };

        let expanded = quote! {
            impl crate::routing::Routable for #name {
                fn route() -> crate::routing::PageRoute {
                    static SIDEBAR: crate::components::sidebar::SidebarContent =
                        crate::components::sidebar::SidebarContent::new(&[
                            #sections
                        ]);

                    crate::routing::PageRoute::Sidebar(crate::routing::SidebarRouteDef {
                        id: #id,
                        label: #label,
                        icon: #icon_expr,
                        sidebar: &SIDEBAR,
                        sidebar_variant: crate::components::sidebar::SidebarVariant::#variant,
                        default_id: #default_id,
                        render_child: <#children_tuple as crate::routing::ChildRoutes>::render,
                    })
                }
            }

            #children_impl
        };

        Ok(expanded)
    }
}
