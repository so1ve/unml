use gpui::prelude::*;
use gpui::*;
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
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let active = self.active;
        let path = format!("{}/{}", self.base_path, self.item.id);

        NavLink::new().to(SharedString::from(path)).child(
            div()
                .id(SharedString::from(self.item.id))
                .h(px(32.0))
                .px_2()
                .rounded(px(4.0))
                .cursor_pointer()
                .flex()
                .items_center()
                .text_color(rgb(if active { 0xe8e8e8 } else { 0xa0a0a0 }))
                .bg(rgb(if active { 0x2d2d2d } else { 0x252525 }))
                .hover(|s| s.bg(rgb(0x2d2d2d)).text_color(rgb(0xe8e8e8)))
                .child(t!(self.item.label).to_string()),
        )
    }
}
