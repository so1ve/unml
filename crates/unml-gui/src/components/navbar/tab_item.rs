use gpui::prelude::*;
use gpui::*;
use gpui_component::Icon;
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
                .child(t!(self.tab.label).to_string()),
        )
    }
}
