use gpui::prelude::*;
use gpui::*;
use gpui_component::ActiveTheme;
use gpui_router::NavLink;
use rust_i18n::t;

use super::SidebarItem;

#[derive(IntoElement)]
pub struct NavItem {
    base_path: &'static str,
    item: &'static SidebarItem,
    active: bool,
}

impl NavItem {
    pub fn new(base_path: &'static str, item: &'static SidebarItem, active: bool) -> Self {
        Self {
            base_path,
            item,
            active,
        }
    }
}

impl RenderOnce for NavItem {
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

        let item_div = div()
            .id(SharedString::from(self.item.id))
            .h(px(40.0))
            .px_3()
            .rounded(px(6.0))
            .border_l_2()
            .cursor_pointer()
            .flex()
            .items_center()
            .text_color(text_color)
            .bg(bg_color)
            .hover(|s| s.bg(theme.list_hover).text_color(theme.foreground))
            .child(t!(self.item.label).to_string());

        let item_div = if active {
            item_div.border_color(theme.list_active_border)
        } else {
            item_div
        };

        NavLink::new().to(SharedString::from(path)).child(item_div)
    }
}
