use gpui::prelude::*;
use gpui::*;
use gpui_router::NavLink;

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
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let active = self.active;
        let path = format!("{}/{}", self.base_path, self.item.id);

        let item_div = div()
            .id(SharedString::from(self.item.id))
            .h(px(40.0))
            .px_3()
            .rounded(px(6.0))
            .border_l_2()
            .cursor_pointer()
            .flex()
            .items_center()
            .text_color(rgb(if active { 0xe8e8e8 } else { 0xa0a0a0 }))
            .bg(rgb(if active { 0x2d2d2d } else { 0x252525 }))
            .hover(|s| s.bg(rgb(0x2d2d2d)).text_color(rgb(0xe8e8e8)))
            .child(SharedString::from(self.item.label));

        let item_div = if active {
            item_div.border_color(rgb(0x3b82f6))
        } else {
            item_div
        };

        NavLink::new().to(SharedString::from(path)).child(item_div)
    }
}
