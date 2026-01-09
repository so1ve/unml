//! Procedural macros for UNML.
//!
//! This crate provides macros for the UNML GUI:
//! - `define_sidebar!` - Define sidebar content, Selection enum, and related
//!   constants

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
