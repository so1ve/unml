//! Procedural macros for UNML.
//!
//! This crate provides derive macros for the UNML GUI:
//!
//! - `#[derive(PageRoute)]` - Derive the `PageRoute` trait for a page struct
//! - `#[derive(SubRoute)]` - Derive the `PageRoute` trait for child/sub pages
//!   (simplified, only path needed)

mod page_route;
mod route_attr;
mod sidebar_attr;
mod sub_route;

use proc_macro::TokenStream;
use syn::DeriveInput;

/// Derive the `PageRoute` trait for a page struct.
///
/// This macro automatically implements the `PageRoute` trait based on the
/// `#[route(...)]` and optional `#[sidebar(...)]` attributes.
///
/// # Attributes
///
/// ## `#[route(...)]` (required)
///
/// Defines the route configuration:
/// - `path = "..."` - Route path (e.g., "/" or "/versions")
/// - `label = "..."` - Navigation label i18n key
/// - `icon = IconName` - Navigation icon (optional)
/// - `home` - Flag indicating this is the home page
///
/// ## `#[sidebar(...)]` (optional)
///
/// Defines the sidebar content:
/// - `variant = Filter | Navigation` - Sidebar variant
/// - `section "title" { ... }` - Section with title
/// - `section { ... }` - Section without title
///
/// The default selected item is always the first item in the first section.
///
/// ## `#[children(...)]` (optional)
///
/// Lists child page types for nested routing.
///
/// # Example
///
/// ```ignore
/// #[derive(PageRoute)]
/// #[route(path = "/versions", label = "nav.versions", icon = Folder)]
/// #[sidebar(
///     variant = Filter,
///     section "versions.filter" {
///         Release => "versions.release",
///         Snapshot => "versions.snapshot",
///     }
/// )]
/// pub struct VersionsPage;
///
/// impl VersionsPage {
///     fn view(_window: &mut Window, _cx: &mut App) -> impl IntoElement {
///         ui! { div { "Versions content" } }
///     }
/// }
/// ```
#[proc_macro_derive(PageRoute, attributes(route, sidebar, children))]
pub fn derive_page_route(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    page_route::derive(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Derive the `PageRoute` trait for a child/sub page.
///
/// This is a simplified version of `#[derive(PageRoute)]` for child pages
/// that only need a path, without label or icon.
///
/// # Attributes
///
/// ## `#[subroute(...)]` (required)
///
/// Defines the sub-route configuration:
/// - `path = "..."` - Route path (e.g., "/settings/java")
///
/// # Example
///
/// ```ignore
/// #[derive(SubRoute)]
/// #[subroute(path = "/settings/java")]
/// pub struct JavaSettingsPage;
///
/// impl JavaSettingsPage {
///     fn view(_window: &mut Window, _cx: &mut App) -> impl IntoElement {
///         ui! { div { "Java settings content" } }
///     }
/// }
/// ```
#[proc_macro_derive(SubRoute, attributes(subroute))]
pub fn derive_sub_route(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    sub_route::derive(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
