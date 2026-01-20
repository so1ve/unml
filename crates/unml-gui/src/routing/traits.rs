//! Core traits for the page routing system.
//!
//! This module defines the [`PageRoute`] trait that all routable pages must
//! implement, along with the [`ChildRoutes`] trait for handling nested routes.

use gpui::{AnyElement, App, IntoElement, Window};
use gpui_component::IconName;

use crate::components::sidebar::{SidebarContent, SidebarVariant};

/// Base trait for all pages that provide a view function.
///
/// This trait defines the core `view` method that all pages must implement.
/// The derive macros for `PageRoute` and `SubRoute` automatically call
/// `Self::view(window, cx).into_any_element()` to implement the `render`
/// method.
///
/// # Example
///
/// ```ignore
/// use crate::routing::PageView;
///
/// impl PageView for HomePage {
///     fn view(_window: &mut Window, _cx: &mut App) -> impl IntoElement {
///         ui! { div { "Home content" } }
///     }
/// }
/// ```
pub trait PageView {
    /// Render the page content.
    ///
    /// This method returns an `impl IntoElement` for flexibility, allowing
    /// pages to return any type that implements `IntoElement`.
    fn view(window: &mut Window, cx: &mut App) -> impl IntoElement;
}

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
/// #[route(id = "", label = "nav.home", icon = LayoutDashboard, home)]
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

    /// Route identifier (e.g., "versions", "settings", "" for home)
    const ID: &'static str;

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

/// Sub-route trait for child pages.
///
/// This trait is for child pages that are rendered within a parent page.
/// Unlike `PageRoute`, sub-routes only need an ID and render function.
///
/// # Example
///
/// ```ignore
/// #[derive(SubRoute)]
/// #[subroute(id = "java")]
/// pub struct JavaSettingsPage;
///
/// impl JavaSettingsPage {
///     fn view(_window: &mut Window, _cx: &mut App) -> impl IntoElement {
///         ui! { div { "Java settings" } }
///     }
/// }
/// ```
pub trait SubRoute: 'static {
    /// Route ID (e.g., "java" for the Java settings page)
    const ID: &'static str;

    /// Render the sub-page content
    fn render(window: &mut Window, cx: &mut App) -> AnyElement;
}

/// Child routes collection trait.
///
/// This trait is implemented for tuples of sub-route types or for `()` when
/// there are no child routes. It provides methods to render child pages by ID.
pub trait ChildRoutes: 'static {
    /// Render a child page by its ID
    fn render(id: &str, window: &mut Window, cx: &mut App) -> Option<AnyElement>;
}

/// Empty child routes implementation
impl ChildRoutes for () {
    fn render(_: &str, _: &mut Window, _: &mut App) -> Option<AnyElement> {
        None
    }
}
