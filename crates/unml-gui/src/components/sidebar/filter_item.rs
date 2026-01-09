use gpui::prelude::*;
use gpui::*;
use gpui_component::ActiveTheme;
use gpui_router::NavLink;
use rust_i18n::t;

use super::SidebarItem;

#[derive(IntoElement)]
pub struct FilterItem {
    base_path: &'static str,
    item: &'static SidebarItem,
    active: bool,
}

impl FilterItem {
    pub fn new(base_path: &'static str, item: &'static SidebarItem, active: bool) -> Self {
        Self {
            base_path,
            item,
            active,
        }
    }
}

impl RenderOnce for FilterItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let active = self.active;
        let path = format!("{}/{}", self.base_path, self.item.id);
        let theme = cx.theme();

        let text_color = if active {
            theme.foreground
        } else {
            theme.muted_foreground
        };
        let bg_color = if active {
            theme.list_active
        } else {
            theme.list
        };

        NavLink::new().to(SharedString::from(path)).child(
            div()
                .id(SharedString::from(self.item.id))
                .h(px(32.0))
                .px_2()
                .rounded(px(4.0))
                .cursor_pointer()
                .flex()
                .items_center()
                .text_color(text_color)
                .bg(bg_color)
                .hover(|s| s.bg(theme.list_hover).text_color(theme.foreground))
                .child(t!(self.item.label).to_string()),
        )
    }
}
