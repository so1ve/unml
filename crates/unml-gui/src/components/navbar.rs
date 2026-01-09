use gpui::prelude::*;
use gpui::*;
use gpui_component::{ActiveTheme, Icon, IconName};
use gpui_router::{NavLink, use_location};

use crate::pages;

/// A tab item definition for the navigation bar
#[derive(Clone)]
pub struct TabItem {
    pub id: &'static str,
    pub label: &'static str,
    pub path: &'static str,
    pub icon: IconName,
}

impl TabItem {
    pub const fn new(id: &'static str, label: &'static str, path: &'static str, icon: IconName) -> Self {
        Self { id, label, path, icon }
    }

    fn is_active(&self, pathname: &str) -> bool {
        if self.path == pages::home::PATH {
            pathname == pages::home::PATH
        } else {
            pathname == self.path || pathname.starts_with(self.path)
        }
    }
}

/// Default navigation tabs
pub const NAV_TABS: &[TabItem] = &[
    TabItem::new("home", "首页", pages::home::PATH, IconName::LayoutDashboard),
    TabItem::new("versions", "版本", pages::versions::PATH, IconName::Folder),
    TabItem::new("mods", "Mod", pages::mods::PATH, IconName::Star),
    TabItem::new("downloads", "下载", pages::downloads::PATH, IconName::ArrowDown),
    TabItem::new("settings", "设置", pages::settings::PATH, IconName::Settings),
];

#[derive(IntoElement)]
pub struct NavBar {
    tabs: &'static [TabItem],
}

impl NavBar {
    pub fn new() -> Self {
        Self { tabs: NAV_TABS }
    }

    pub fn with_tabs(mut self, tabs: &'static [TabItem]) -> Self {
        self.tabs = tabs;
        self
    }
}

impl RenderOnce for NavBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let pathname = use_location(cx).pathname.clone();

        div()
            .id("navbar")
            .h(px(48.0))
            .w_full()
            .bg(rgb(0x252525))
            .border_b_1()
            .border_color(cx.theme().border)
            .flex()
            .items_center()
            .px_4()
            .gap_1()
            .children(
                self.tabs
                    .iter()
                    .map(|tab| TabItemView::new(tab.clone(), pathname.clone()).into_any_element()),
            )
    }
}

/// Renders a single tab item
#[derive(IntoElement)]
struct TabItemView {
    tab: TabItem,
    pathname: SharedString,
}

impl TabItemView {
    fn new(tab: TabItem, pathname: SharedString) -> Self {
        Self { tab, pathname }
    }
}

impl RenderOnce for TabItemView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let active = self.tab.is_active(&self.pathname);
        let path = SharedString::from(self.tab.path);

        NavLink::new().to(path.clone()).child(
            div()
                .id(SharedString::from(self.tab.id))
                .h(px(36.0))
                .px_4()
                .rounded(px(6.0))
                .cursor_pointer()
                .flex()
                .items_center()
                .gap_2()
                .text_color(rgb(if active { 0xe8e8e8 } else { 0xa0a0a0 }))
                .bg(rgb(if active { 0x2d2d2d } else { 0x252525 }))
                .hover(|s| s.bg(rgb(0x2d2d2d)).text_color(rgb(0xe8e8e8)))
                .active(|s| s.bg(rgb(0x353535)))
                .when(active, |s| {
                    s.border_b_2().border_color(rgb(0x3b82f6)).rounded_b_none()
                })
                .child(
                    Icon::new(self.tab.icon)
                        .size_4()
                        .text_color(rgb(if active { 0xe8e8e8 } else { 0xa0a0a0 })),
                )
                .child(SharedString::from(self.tab.label)),
        )
    }
}
