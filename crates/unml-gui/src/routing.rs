//! Routing system for the application.
//!
//! This module provides a type-safe routing system based on the [`PageRoute`]
//! trait. Pages implement this trait (typically via the `#[derive(PageRoute)]`
//! macro) to define their route configuration alongside their implementation.
//!
//! # Architecture
//!
//! The routing system consists of:
//! - [`Page`] trait - Base trait providing the `view` method
//! - [`PageRoute`] trait - Defines route configuration for a page
//! - [`SubRoute`] trait - Defines child/sub-route configuration
//! - [`ChildRoutes`] trait - Handles nested child routes
//! - [`build_route`] - Builds a `Route` from a `PageRoute` implementation
//! - [`routes!`] macro - Generates router from page types
//! - [`nav_tabs!`] macro - Generates navigation tabs from page types
//!
//! # Example
//!
//! ```ignore
//! use unml_macros::PageRoute;
//! use crate::routing::PageView;
//!
//! #[derive(PageRoute)]
//! #[route(id = "versions", label = "nav.versions", icon = Folder)]
//! #[sidebar(
//!     variant = Filter,
//!     section "versions.filter" {
//!         Release => "versions.release",
//!         Snapshot => "versions.snapshot",
//!     }
//! )]
//! pub struct VersionsPage;
//!
//! impl PageView for VersionsPage {
//!     fn view(_window: &mut Window, _cx: &mut App) -> impl IntoElement {
//!         ui! { div { "Versions content" } }
//!     }
//! }
//! ```

mod builder;
mod traits;

pub use builder::build_route;
pub use traits::{ChildRoutes, PageRoute, PageView, SubRoute};

/// Define routes and navigation tabs from a list of page types.
///
/// This macro generates both the `router()` function and `NAV_TABS` constant
/// from the same list of page types. Each page type must implement
/// [`PageRoute`].
///
/// # Example
///
/// ```ignore
/// use crate::pages::{HomePage, VersionsPage, ModsPage};
///
/// define_routes![
///     HomePage,
///     VersionsPage,
///     ModsPage,
/// ];
///
/// // This generates:
/// // - pub fn router() -> impl IntoElement { ... }
/// // - pub const NAV_TABS: &[TabItem] = ...;
/// ```
#[macro_export]
macro_rules! define_routes {
    ($($page:ty),* $(,)?) => {
        /// Build the application router.
        pub fn router() -> impl gpui::IntoElement {
            gpui_router::Routes::new()
                .basename("/")
                $(.child($crate::routing::build_route::<$page>()))*
        }

        /// Navigation tabs for the navbar.
        pub const NAV_TABS: &[$crate::components::navbar::TabItem] =
            &[$($crate::components::navbar::TabItem::from_page::<$page>()),*];
    };
}
