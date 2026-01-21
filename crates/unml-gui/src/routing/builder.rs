//! Route builder utilities.

use gpui_router::{Route, use_params};

use super::traits::{PageRoute, PlainRouteDef, Routable, SidebarRouteDef};
use crate::components::layout::{HomeLayout, PageLayout};

pub fn build_route<T: Routable>() -> Route {
    match T::route() {
        PageRoute::Plain(def) => build_plain_route(def),
        PageRoute::Sidebar(def) => build_sidebar_route(def),
    }
}

fn build_plain_route(def: PlainRouteDef) -> Route {
    Route::new()
        .path(def.id)
        .layout(HomeLayout::new())
        .child(Route::new().index().element(def.render))
}

fn build_sidebar_route(def: SidebarRouteDef) -> Route {
    let default_id = def.default_id;
    let render_child = def.render_child;

    Route::new()
        .path(def.id)
        .layout(PageLayout::new(
            def.id,
            def.sidebar,
            def.sidebar_variant,
            def.default_id,
        ))
        .child(Route::new().path("{subroute}").element(move |window, cx| {
            let subroute: String = use_params(cx)
                .get("subroute")
                .map(|s| s.to_string())
                .unwrap_or_else(|| default_id.to_string());

            render_child(&subroute, window, cx)
        }))
}
