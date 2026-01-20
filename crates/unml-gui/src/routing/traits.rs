//! Core traits for the page routing system.
//!
//! This module defines the [`PageRoute`] trait that all routable pages must
//! implement, along with the [`ChildRoutes`] trait for handling nested routes.

use gpui::{AnyElement, App, Window};
use gpui_component::IconName;

use crate::components::sidebar::{SidebarContent, SidebarVariant};

/// Page route trait that all routable pages must implement.
///
/// This trait provides a type-safe way to define route configuration alongside
/// the page implementation. Use the `#[derive(PageRoute)]` macro to implement
/// this trait automatically.
///
/// # Example
///
/// ```ignore
/// #[derive(PageRoute)]
/// #[route(path = "/", label = "nav.home", icon = LayoutDashboard, home)]
/// pub struct HomePage;
///
/// impl HomePage {
///     fn view(_window: &mut Window, _cx: &mut App) -> impl IntoElement {
///         ui! { div { "Home content" } }
///     }
/// }
/// ```
pub trait PageRoute: 'static {
    /// Child routes type (for nested routing)
    /// Defaults to `()` when not specified in the derive macro
    type Children: ChildRoutes;

    /// Route path, e.g. "/" or "/versions"
    const PATH: &'static str;

    /// Route ID, the last segment of PATH (e.g., "java" for "/settings/java")
    const ID: &'static str = "";

    /// Navigation label i18n key
    const LABEL: &'static str;

    /// Navigation icon (optional - child routes may not have icons)
    const ICON: Option<IconName> = None;

    /// Whether this is the home page (uses HomeLayout)
    const IS_HOME: bool = false;

    /// Sidebar content
    const SIDEBAR: Option<&'static SidebarContent> = None;

    /// Sidebar variant
    const SIDEBAR_VARIANT: Option<SidebarVariant> = None;

    /// Default sub-route ID (first item of first section for pages with
    /// sidebar)
    const DEFAULT_ID: &'static str = "";

    /// Render the page content
    fn render(window: &mut Window, cx: &mut App) -> AnyElement;
}

/// Child routes collection trait.
///
/// This trait is implemented for tuples of page types or for `()` when there
/// are no child routes. It provides methods to render child pages by ID and
/// list all available child IDs.
pub trait ChildRoutes: 'static {
    /// Render a child page by its ID
    fn render(id: &str, window: &mut Window, cx: &mut App) -> Option<AnyElement>;

    /// Get all child route IDs
    #[allow(dead_code)]
    fn ids() -> &'static [&'static str];
}

/// Empty child routes implementation
impl ChildRoutes for () {
    fn render(_: &str, _: &mut Window, _: &mut App) -> Option<AnyElement> {
        None
    }

    fn ids() -> &'static [&'static str] {
        &[]
    }
}
