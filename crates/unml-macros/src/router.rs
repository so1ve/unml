//! Implementation of the `define_app_routes!` macro.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitStr, Result, Token, braced};

struct ChildRoute {
    name: Ident,
}

impl Parse for ChildRoute {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;

        Ok(ChildRoute { name })
    }
}

struct RouteDef {
    name: Ident,
    label: LitStr,
    icon: Ident,
    children: Vec<ChildRoute>,
}

impl Parse for RouteDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;

        let content;
        braced!(content in input);

        // Parse `label: "..."``
        let label_ident: Ident = content.parse()?;
        if label_ident != "label" {
            return Err(syn::Error::new(label_ident.span(), "expected `label`"));
        }
        content.parse::<Token![:]>()?;
        let label: LitStr = content.parse()?;
        content.parse::<Token![,]>()?;

        // Parse `icon: Ident`
        let icon_ident: Ident = content.parse()?;
        if icon_ident != "icon" {
            return Err(syn::Error::new(icon_ident.span(), "expected `icon`"));
        }
        content.parse::<Token![:]>()?;
        let icon: Ident = content.parse()?;

        if content.peek(Token![,]) {
            content.parse::<Token![,]>()?;
        }

        // children block
        let mut children = Vec::new();
        if !content.is_empty() {
            let children_ident: Ident = content.parse()?;
            if children_ident != "children" {
                return Err(syn::Error::new(
                    children_ident.span(),
                    "expected `children`",
                ));
            }

            let children_content;
            braced!(children_content in content);

            while !children_content.is_empty() {
                children.push(children_content.parse()?);
                if children_content.peek(Token![,]) {
                    children_content.parse::<Token![,]>()?;
                }
            }
        }

        Ok(RouteDef {
            name,
            label,
            icon,
            children,
        })
    }
}

struct RoutesDef {
    home: RouteDef,
    routes: Vec<RouteDef>,
}

impl Parse for RoutesDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let home: RouteDef = input.parse()?;
        if home.name != "home" {
            return Err(syn::Error::new(
                home.name.span(),
                "first route must be `home`",
            ));
        }

        let mut routes = Vec::new();
        while !input.is_empty() {
            routes.push(input.parse()?);
        }

        Ok(RoutesDef { home, routes })
    }
}

/// Convert snake_case to SCREAMING_SNAKE_CASE
fn to_screaming_snake_case(s: &str) -> String {
    s.to_uppercase()
}

pub fn define(input: TokenStream) -> TokenStream {
    let def: RoutesDef = match syn::parse2(input) {
        Ok(def) => def,
        Err(err) => return err.to_compile_error(),
    };

    let home_label = &def.home.label;
    let home_icon = &def.home.icon;

    let mut path_consts = vec![quote! {
        pub const HOME: &str = "/";
    }];

    for route in &def.routes {
        let name_upper = format_ident!("{}", to_screaming_snake_case(&route.name.to_string()));
        let name_str = route.name.to_string();
        let path = format!("/{}", name_str);

        path_consts.push(quote! {
            pub const #name_upper: &str = #path;
        });

        for child in &route.children {
            let child_name = &child.name;
            let child_name_str = child_name.to_string();
            let combined_name = format_ident!(
                "{}_{}",
                to_screaming_snake_case(&name_str),
                to_screaming_snake_case(&child_name_str)
            );
            let child_path = format!("/{}/{}", name_str, child_name_str);

            path_consts.push(quote! {
                pub const #combined_name: &str = #child_path;
            });
        }
    }

    let route_children: Vec<_> = def
        .routes
        .iter()
        .map(|route| {
            let name = &route.name;
            let name_upper = format_ident!("{}", to_screaming_snake_case(&name.to_string()));

            if route.children.is_empty() {
                quote! {
                    .child(
                        Route::new()
                            .path(stringify!(#name))
                            .layout(PageLayout::new(
                                paths::#name_upper,
                                pages::#name::SIDEBAR.unwrap(),
                                pages::#name::VARIANT.unwrap(),
                                pages::#name::DEFAULT_ID.unwrap(),
                            ))
                            .child(Route::new().index().element(|_, _| pages::#name::page()))
                            .child(
                                Route::new()
                                    .path("{subroute}")
                                    .element(|_, _| pages::#name::page()),
                            )
                    )
                }
            } else {
                let first_child = &route.children[0].name;
                let child_dispatch_arms: Vec<_> = route
                    .children
                    .iter()
                    .map(|child| {
                        let child_name = &child.name;
                        let child_name_str = child_name.to_string();
                        quote! {
                            #child_name_str => pages::#name::#child_name::page(window, cx).into_any_element()
                        }
                    })
                    .collect();

                quote! {
                    .child(
                        Route::new()
                            .path(stringify!(#name))
                            .layout(PageLayout::new(
                                paths::#name_upper,
                                pages::#name::SIDEBAR.unwrap(),
                                pages::#name::VARIANT.unwrap(),
                                pages::#name::DEFAULT_ID.unwrap(),
                            ))
                            .child(Route::new().index().element(|_, _| pages::#name::page()))
                            .child(
                                Route::new()
                                    .path("{subroute}")
                                    .element(|window, cx| {
                                        let params = gpui_router::use_params(cx);
                                        let subroute = params.get("subroute").map(|s| s.as_str()).unwrap_or(stringify!(#first_child));
                                        match subroute {
                                            #(#child_dispatch_arms,)*
                                            _ => unreachable!("invalid subroute! how did we get here? got {subroute}"),
                                        }
                                    })
                            )
                    )
                }
            }
        })
        .collect();

    let nav_tabs: Vec<_> = def
        .routes
        .iter()
        .map(|route| {
            let name = &route.name;
            let name_upper = format_ident!("{}", to_screaming_snake_case(&name.to_string()));
            let label = &route.label;
            let icon = &route.icon;

            quote! {
                TabItem::new(
                    stringify!(#name),
                    #label,
                    paths::#name_upper,
                    pages::#name::DEFAULT_ID,
                    IconName::#icon,
                )
            }
        })
        .collect();

    quote! {
        use gpui::IntoElement;
        use gpui_component::IconName;
        use gpui_router::{Route, Routes};

        use crate::components::layout::{HomeLayout, PageLayout};
        use crate::components::navbar::TabItem;
        use crate::pages;

        pub mod paths {
            #(#path_consts)*
        }

        pub fn router() -> impl IntoElement {
            Routes::new()
                .basename("/")
                .child(
                    Route::new()
                        .index()
                        .layout(HomeLayout::new())
                        .child(Route::new().index().element(|_, _| pages::home::page())),
                )
                #(#route_children)*
        }

        pub const NAV_TABS: &[TabItem] = &[
            TabItem::new(
                "home",
                #home_label,
                paths::HOME,
                None,
                IconName::#home_icon,
            ),
            #(#nav_tabs),*
        ];
    }
}
