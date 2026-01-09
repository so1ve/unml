use gpui::prelude::*;
use gpui::*;
use gpui_component::ActiveTheme;

// ============================================================================
// Page Path
// ============================================================================

pub const PATH: &str = "/";

// ============================================================================
// Account Panel Sidebar
// ============================================================================

/// Account panel sidebar for home page
#[derive(IntoElement)]
pub struct Sidebar;

impl Sidebar {
    pub fn new() -> Self {
        Self
    }
}

impl RenderOnce for Sidebar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .id("account-panel")
            .flex()
            .flex_col()
            .items_center()
            .p_4()
            .gap_4()
            // Avatar placeholder (96x96)
            .child(
                div()
                    .w(px(96.0))
                    .h(px(96.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x3b82f6))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(rgb(0xffffff))
                    .text_xl()
                    .child(SharedString::from("S")),
            )
            // Username
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgb(0xe8e8e8))
                    .child(SharedString::from("Steve")),
            )
            // Account type label
            .child(
                div()
                    .text_sm()
                    .text_color(cx.theme().muted_foreground)
                    .child(SharedString::from("微软账号")),
            )
            // Account selector
            .child(
                div()
                    .w_full()
                    .px_2()
                    .child(
                        div()
                            .w_full()
                            .h(px(36.0))
                            .px_3()
                            .rounded(px(6.0))
                            .bg(rgb(0x2d2d2d))
                            .border_1()
                            .border_color(cx.theme().border)
                            .hover(|s| s.bg(rgb(0x353535)))
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .justify_between()
                            .text_color(rgb(0xe8e8e8))
                            .child(SharedString::from("Steve (微软)"))
                            .child(SharedString::from("▼")),
                    ),
            )
            // Divider
            .child(div().w_full().h(px(1.0)).bg(cx.theme().border).my_2())
            // Add account button
            .child(
                div()
                    .w_full()
                    .px_2()
                    .child(
                        div()
                            .w_full()
                            .h(px(36.0))
                            .rounded(px(6.0))
                            .bg(rgb(0x2d2d2d))
                            .hover(|s| s.bg(rgb(0x3d3d3d)))
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .justify_center()
                            .gap_2()
                            .text_color(rgb(0xa0a0a0))
                            .child(SharedString::from("+ 添加账号")),
                    ),
            )
    }
}

#[derive(IntoElement)]
pub struct Page;

impl RenderOnce for Page {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(10.0))
            .child(div().text_size(px(20.0)).child(SharedString::from("首页")))
            .child(
                div()
                    .text_color(rgb(0xa0a0a0))
                    .child(SharedString::from("游戏实例列表")),
            )
    }
}

pub fn page() -> Page {
    Page
}
