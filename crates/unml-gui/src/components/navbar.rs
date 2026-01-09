use gpui::prelude::*;
use gpui::*;
use gpui_component::{ActiveTheme, Icon, IconName};
use gpui_router::{NavLink, use_location};

use crate::routes::{NAV_TABS, paths};

/// A tab item definition for the navigation bar
#[derive(Clone)]
pub struct TabItem {
    pub id: &'static str,
    pub label: &'static str,
    /// Path prefix used to determine active state (base path like "/versions")
    pub active_prefix: &'static str,
    /// Default selection id (e.g. "release"), used to build link target
    /// `/{active_prefix}/{default_id}`. If None, link target is
    /// `active_prefix`.
    pub default_id: Option<&'static str>,
    pub icon: IconName,
}

impl TabItem {
    pub const fn new(
        id: &'static str,
        label: &'static str,
        active_prefix: &'static str,
        default_id: Option<&'static str>,
        icon: IconName,
    ) -> Self {
        Self {
            id,
            label,
            active_prefix,
            default_id,
            icon,
        }
    }

    fn is_active(&self, pathname: &str) -> bool {
        if self.active_prefix == paths::HOME {
            return pathname == paths::HOME;
        }

        if !pathname.starts_with(self.active_prefix) {
            return false;
        }

        // Ensure path boundary: "/versions" matches "/versions" and "/versions/...",
        // but not "/versions2".
        let prefix_len = self.active_prefix.len();
        pathname.len() == prefix_len || pathname.as_bytes().get(prefix_len) == Some(&b'/')
    }
}

#[derive(IntoElement)]
pub struct NavBar {
    tabs: &'static [TabItem],
}

impl NavBar {
    pub fn new() -> Self {
        Self { tabs: NAV_TABS }
    }

    #[allow(dead_code)]
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
        let to = match self.tab.default_id {
            Some(default_id) => {
                SharedString::from(format!("{}/{}", self.tab.active_prefix, default_id))
            }
            None => SharedString::from(self.tab.active_prefix),
        };

        NavLink::new().to(to).child(
            div()
                .id(SharedString::from(self.tab.id))
                .h(px(36.0))
                .px_4()
                .border_b_2()
                .rounded(px(6.0))
                .cursor_pointer()
                .flex()
                .items_center()
                .gap_2()
                .text_color(rgb(if active { 0xe8e8e8 } else { 0xa0a0a0 }))
                .bg(rgb(if active { 0x2d2d2d } else { 0x252525 }))
                .hover(|s| s.bg(rgb(0x2d2d2d)).text_color(rgb(0xe8e8e8)))
                .active(|s| s.bg(rgb(0x353535)))
                .when(active, |s| s.border_color(rgb(0x3b82f6)).rounded_b_none())
                .child(Icon::new(self.tab.icon).size_4().text_color(rgb(if active {
                    0xe8e8e8
                } else {
                    0xa0a0a0
                })))
                .child(SharedString::from(self.tab.label)),
        )
    }
}
