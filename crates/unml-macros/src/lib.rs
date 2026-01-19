//! Procedural macros for UNML.
//!
//! This crate provides macros for the UNML GUI:
//! - `define_sidebar!` - Define sidebar content, Selection enum, and related
//!   constants
//! - `define_app_routes!` - Define application routes and navigation tabs

mod router;
mod sidebar;

use proc_macro::TokenStream;

/// Macro to define sidebar content and Selection enum.
///
/// Generates:
/// - `Selection` enum with variants
/// - `Selection::id()`, `Selection::from_id()`
/// - `SIDEBAR: Option<&'static SidebarContent>`
/// - `VARIANT: Option<SidebarVariant>`
/// - `DEFAULT_ID: Option<&'static str>`
///
/// # Example
///
/// ```ignore
/// define_sidebar! {
///     variant: Filter,
///
///     section "筛选" {
///         Release => "正式版",
///         Snapshot => "快照版",
///     }
///     section {
///         Installed => "仅已安装",
///     }
/// }
/// ```
///
/// For pages without a sidebar:
///
/// ```ignore
/// define_sidebar! {}
/// ```
#[proc_macro]
pub fn define_sidebar(input: TokenStream) -> TokenStream {
    sidebar::define(input.into()).into()
}

/// Macro to define application routes and navigation tabs.
///
/// Generates:
/// - `pub mod paths` - PATH constants for each route
/// - `pub fn router()` - the application router
/// - `pub const NAV_TABS: &[TabItem]` - navigation tabs for the navbar
///
/// # Syntax
///
/// ```ignore
/// define_app_routes! {
///     // Home route (special: path is "/")
///     home {
///         label: "nav.home",
///         icon: LayoutDashboard,
///     }
///
///     // Simple route without children
///     versions {
///         label: "nav.versions",
///         icon: Folder,
///     }
///
///     // Route with children (file-based routing)
///     settings {
///         label: "nav.settings",
///         icon: Settings,
///
///         children {
///             java,       // -> /settings/java -> pages::settings::java::page()
///             general,    // -> /settings/general -> pages::settings::general::page()
///         }
///     }
/// }
/// ```
///
/// # Path Generation Rules
///
/// | Route       | Generated Path    | Page Module                      |
/// |-------------|-------------------|----------------------------------|
/// | `home`      | `/`               | `pages::home::page()`            |
/// | `versions`  | `/versions`       | `pages::versions::page()`        |
/// | `settings`  | `/settings`       | `pages::settings::page()`        |
/// | `java` (child of settings) | `/settings/java` | `pages::settings::java::page()` |
#[proc_macro]
pub fn define_app_routes(input: TokenStream) -> TokenStream {
    router::define(input.into()).into()
}
