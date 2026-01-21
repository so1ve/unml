//! Core traits for the page routing system.

use gpui::{AnyElement, App, IntoElement, Window};
use gpui_component::IconName;

use crate::components::sidebar::{SidebarContent, SidebarVariant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageKind {
    Home,
    Page,
}

/// Base trait for pages that provide a `view` function.
pub trait PageView {
    fn view(window: &mut Window, cx: &mut App) -> impl IntoElement;
}

/// Page route trait for routable pages.
pub trait PageRoute: 'static {
    type Children: ChildRoutes;

    const ID: &'static str;
    const LABEL: &'static str;
    const ICON: Option<IconName> = None;
    const KIND: PageKind = PageKind::Page;
    const SIDEBAR: Option<&'static SidebarContent> = None;
    const SIDEBAR_VARIANT: Option<SidebarVariant> = None;
    const DEFAULT_ID: &'static str = "";

    fn render(window: &mut Window, cx: &mut App) -> AnyElement;
}

/// Sub-route trait for child pages.
pub trait SubRoute: 'static {
    const ID: &'static str;
    const TITLE: Option<&'static str> = None;

    fn render(window: &mut Window, cx: &mut App) -> AnyElement;
}

/// Child routes collection trait.
pub trait ChildRoutes: 'static {
    fn render(id: &str, window: &mut Window, cx: &mut App) -> AnyElement;
}

impl ChildRoutes for () {
    fn render(_: &str, _: &mut Window, _: &mut App) -> AnyElement {
        unreachable!("Pages without children should not use subroute paths")
    }
}
