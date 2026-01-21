//! Route builder utilities.

use gpui_router::{Route, use_params};

use super::traits::{ChildRoutes, PageKind, PageRoute};
use crate::components::layout::{HomeLayout, PageLayout};

pub fn build_route<P: PageRoute>() -> Route {
    match P::KIND {
        PageKind::Home => build_home_route::<P>(),
        PageKind::Page => build_page_route::<P>(),
    }
}

fn build_home_route<P: PageRoute>() -> Route {
    Route::new()
        .index()
        .layout(HomeLayout::new())
        .child(Route::new().index().element(|w, cx| P::render(w, cx)))
}

fn build_page_route<P: PageRoute>() -> Route {
    let route = Route::new().path(P::ID);

    let route = if let (Some(sidebar), Some(variant)) = (P::SIDEBAR, P::SIDEBAR_VARIANT) {
        route.layout(PageLayout::new(P::ID, sidebar, variant, P::DEFAULT_ID))
    } else {
        route
    };

    route.child(Route::new().path("{subroute}").element(|window, cx| {
        let subroute: String = use_params(cx)
            .get("subroute")
            .map(|s| s.to_string())
            .unwrap_or_else(|| P::DEFAULT_ID.to_string());

        P::Children::render(&subroute, window, cx)
    }))
}
