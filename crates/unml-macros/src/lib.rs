//! Procedural macros for UNML.

mod layout_attr;
mod page_route;
mod route_attr;
mod sidebar_attr;
mod sub_route;

use proc_macro::TokenStream;
use syn::DeriveInput;

/// Derive the `PageRoute` trait for a page struct.
///
/// # Required Attributes
///
/// - `#[route(id = "...", label = "...", icon = IconName)]`
/// - Add `home` flag for the home page
///
/// # Optional Attributes
///
/// - `#[sidebar(variant = Filter|Navigation, section "title" { ... })]`
/// - `#[children(ChildPage1, ChildPage2)]`
#[proc_macro_derive(PageRoute, attributes(route, layout, sidebar, children))]
pub fn derive_page_route(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    page_route::derive(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Derive the `SubRoute` trait for a child/sub page.
///
/// # Required Attributes
///
/// - `#[subroute(id = "...")]`
#[proc_macro_derive(SubRoute, attributes(subroute, layout))]
pub fn derive_sub_route(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    sub_route::derive(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
