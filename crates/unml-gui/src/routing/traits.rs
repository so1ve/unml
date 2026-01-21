//! Core traits for the page routing system.

use gpui::{AnyElement, App, IntoElement, Window};
use gpui_component::IconName;

use crate::components::sidebar::{SidebarContent, SidebarVariant};

/// Plain route definition for simple pages without sidebar.
pub struct PlainRouteDef {
    pub id: &'static str,
    pub label: &'static str,
    pub icon: Option<IconName>,
    pub render: fn(&mut Window, &mut App) -> AnyElement,
}

/// Sidebar route definition for pages with sidebar navigation.
pub struct SidebarRouteDef {
    pub id: &'static str,
    pub label: &'static str,
    pub icon: Option<IconName>,
    pub sidebar: &'static SidebarContent,
    pub sidebar_variant: SidebarVariant,
    pub default_id: &'static str,
    pub render_child: fn(&str, &mut Window, &mut App) -> AnyElement,
}

pub enum PageRoute {
    Plain(PlainRouteDef),
    Sidebar(SidebarRouteDef),
}

pub trait Routable: 'static {
    fn route() -> PageRoute;
}

/// Base trait for pages that provide a `view` function.
pub trait PageView {
    fn view(window: &mut Window, cx: &mut App) -> impl IntoElement;
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
