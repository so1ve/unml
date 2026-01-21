use gpui::prelude::*;
use gpui::*;
use gpui_component::{ActiveTheme, Icon};
use gpui_markup::ui;
use gpui_router::NavLink;
use rust_i18n::t;

use super::TabItem;

#[derive(IntoElement)]
pub struct TabItemView {
    tab: TabItem,
    pathname: SharedString,
}

impl TabItemView {
    pub fn new(tab: TabItem, pathname: SharedString) -> Self {
        Self { tab, pathname }
    }
}

impl RenderOnce for TabItemView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let active = self.tab.is_active(&self.pathname);

        let to = if self.tab.active_id.is_empty() {
            SharedString::from("/")
        } else {
            SharedString::from(format!("/{}/{}", self.tab.active_id, self.tab.default_id))
        };

        let theme = cx.theme();
        let text_color = if active {
            theme.tab_active_foreground
        } else {
            theme.tab_foreground
        };
        let bg_color = if active { theme.tab_active } else { theme.tab };
        let tab_id = SharedString::from(self.tab.id);
        let label = t!(self.tab.label).to_string();

        let icon_element = self.tab.icon.map(|icon| {
            Icon::new(icon)
                .size_4()
                .text_color(text_color)
                .into_any_element()
        });

        ui! {
            NavLink @[to: to] {
                div @[
                    id: tab_id,
                    h: px(36.0),
                    px_4,
                    border_b_2,
                    rounded: px(6.0),
                    cursor_pointer,
                    flex,
                    items_center,
                    gap_2,
                    text_color: text_color,
                    bg: bg_color,
                    hover: |s| s.bg(theme.list_hover).text_color(theme.foreground),
                    when: (active, |s| s.border_color(theme.primary).rounded_b_none())
                ] {
                    ..icon_element,
                    label
                }
            }
        }
    }
}
