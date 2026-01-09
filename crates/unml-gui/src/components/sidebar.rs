use gpui::prelude::*;
use gpui::*;
use gpui_router::{NavLink, use_location};

#[derive(IntoElement, Clone, Copy)]
pub struct Sidebar;

impl Sidebar {
    pub fn new() -> Self {
        Self
    }
}

impl RenderOnce for Sidebar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let pathname = use_location(cx).pathname.clone();

        div()
            .id("sidebar")
            .w(px(200.0))
            .bg(rgb(0x252525))
            .text_color(rgb(0xa0a0a0))
            .p(px(12.0))
            .child(nav_item("首页", "/", pathname.as_ref()))
            .child(nav_item("版本管理", "/versions", pathname.as_ref()))
            .child(nav_item("Mod 管理", "/mods", pathname.as_ref()))
            .child(nav_item("下载中心", "/downloads", pathname.as_ref()))
            .child(nav_item("设置", "/settings", pathname.as_ref()))
    }
}

fn nav_item(label: &str, to: &str, pathname: &str) -> impl IntoElement {
    let active = pathname == to;

    let to = SharedString::from(to.to_owned());

    NavLink::new().to(to.clone()).child(
        div()
            .id(to.clone())
            .h(px(48.0))
            .rounded(px(6.0))
            .p(px(12.0))
            .cursor_pointer()
            .text_color(rgb(if active { 0xe8e8e8 } else { 0xa0a0a0 }))
            .bg(rgb(if active { 0x2d2d2d } else { 0x252525 }))
            .hover(|s| s.bg(rgb(if active { 0x303030 } else { 0x2a2a2a })))
            .active(|s| s.bg(rgb(if active { 0x353535 } else { 0x303030 })))
            .child(SharedString::from(label.to_owned())),
    )
}
