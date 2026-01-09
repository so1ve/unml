use gpui::prelude::*;
use gpui::*;

// ============================================================================
// Page Path
// ============================================================================

pub const PATH: &str = "/";

// ============================================================================
// Page Content
// ============================================================================

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
