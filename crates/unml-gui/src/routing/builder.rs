//! Route builder utilities.
//!
//! This module provides helper functions for building routes from [`PageRoute`]
//! implementations.

use gpui_router::{Route, use_params};

use super::traits::{ChildRoutes, PageRoute};
use crate::components::layout::{HomeLayout, PageLayout};

/// Build a route from a [`PageRoute`] type.
///
/// This function creates the appropriate route configuration based on the
/// page's trait implementation:
/// - Home pages use `HomeLayout`
/// - Pages with sidebars use `PageLayout`
/// - Pages with children set up dynamic child routing
///
/// # Example
///
/// ```ignore
/// use crate::routing::builder::build_route;
///
/// let route = build_route::<VersionsPage>();
/// ```
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
    let path = P::PATH.trim_start_matches('/');

    let route = Route::new().path(path);

    let route = if let (Some(sidebar), Some(variant)) = (P::SIDEBAR, P::SIDEBAR_VARIANT) {
        route.layout(PageLayout::new(P::PATH, sidebar, variant, P::DEFAULT_ID))
    } else {
        route
    };

    route
        .child(Route::new().index().element(|w, cx| P::render(w, cx)))
        .child(Route::new().path("{subroute}").element(|window, cx| {
            let subroute: String = use_params(cx)
                .get("subroute")
                .map(|s| s.to_string())
                .unwrap_or_else(|| P::DEFAULT_ID.to_string());

            if let Some(element) = P::Children::render(&subroute, window, cx) {
                element
            } else {
                P::render(window, cx)
            }
        }))
}
