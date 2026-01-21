//! Route builder utilities.

use gpui::IntoElement;
use gpui_router::{Route, use_params};

use super::traits::{ChildRoutes, PageRoute};
use crate::components::layout::{HomeLayout, PageContent, PageLayout};

pub fn build_route<P: PageRoute>() -> Route {
    if P::IS_HOME {
        build_home_route::<P>()
    } else {
        build_page_route::<P>()
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

    route
        .child(Route::new().index().element(|w, cx| {
            PageContent::new(P::TITLE, P::render(w, cx)).into_any_element()
        }))
        .child(Route::new().path("{subroute}").element(|window, cx| {
            let subroute: String = use_params(cx)
                .get("subroute")
                .map(|s| s.to_string())
                .unwrap_or_else(|| P::DEFAULT_ID.to_string());

            if let Some(element) = P::Children::render(&subroute, window, cx) {
                element
            } else {
                PageContent::new(P::TITLE, P::render(window, cx)).into_any_element()
            }
        }))
}
