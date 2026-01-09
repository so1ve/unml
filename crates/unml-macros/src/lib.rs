//! Procedural macros for UNML.
//!
//! This crate provides the `Selection` derive macro for generating
//! selection enums with const default and id methods.

mod selection;

use proc_macro::TokenStream;

/// Derive macro for selection enums.
///
/// Generates:
/// - `const fn default() -> Self` - returns the variant marked with
///   `#[default]`
/// - `const fn id(&self) -> &'static str` - returns the id from `#[id = "..."]`
/// - `fn from_id(id: &str) -> Self` - parses an id string to the enum
/// - `impl Default for Selection` - delegates to `Self::default()`
///
/// # Example
///
/// ```ignore
/// use unml_macros::Selection;
///
/// #[derive(Clone, Copy, PartialEq, Eq, Selection)]
/// pub enum Selection {
///     #[default]
///     #[id = "release"]
///     Release,
///     #[id = "snapshot"]
///     Snapshot,
/// }
///
/// // You can then define:
/// pub const DEFAULT_ID: &str = Selection::default().id();
/// ```
#[proc_macro_derive(Selection, attributes(default, id))]
pub fn derive_selection(input: TokenStream) -> TokenStream {
    selection::derive(input.into()).into()
}
