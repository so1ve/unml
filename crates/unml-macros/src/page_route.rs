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
    let is_home = route_attr.is_home;

    // Validate: Page type (non-home) must have children
    if !is_home && children.is_empty() {
        return Err(syn::Error::new(
            input.ident.span(),
            "Page routes require at least one child route. Add #[children(...)] attribute.",
        ));
    }

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

    if is_home {
        let kind_const = quote! {
            const KIND: crate::routing::PageKind = crate::routing::PageKind::Home;
        };

        let expanded = quote! {
            impl crate::routing::PageRoute for #name {
                type Children = ();

                const ID: &'static str = #id;
                const LABEL: &'static str = #label;

                #icon_const
                #kind_const

                fn render(window: &mut gpui::Window, cx: &mut gpui::App) -> gpui::AnyElement {
                    gpui::IntoElement::into_any_element(<Self as crate::routing::PageView>::view(window, cx))
                }
            }
        };

        Ok(expanded)
    } else {
        let kind_const = quote! {
            const KIND: crate::routing::PageKind = crate::routing::PageKind::Page;
        };

        let child_types: Vec<_> = children.iter().collect();
        let children_type = quote! { type Children = (#(#child_types,)*); };

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

        let render_fn = quote! {
            fn render(window: &mut gpui::Window, cx: &mut gpui::App) -> gpui::AnyElement {
                <#first_child as crate::routing::SubRoute>::render(window, cx)
            }
        };

        let expanded = quote! {
            impl crate::routing::PageRoute for #name {
                #children_type

                const ID: &'static str = #id;
                const LABEL: &'static str = #label;

                #icon_const
                #kind_const
                #sidebar_const
                #variant_const
                #default_id_const

                #render_fn
            }

            #children_impl
        };

        Ok(expanded)
    }
}
