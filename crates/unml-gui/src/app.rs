use gpui::*;

use crate::components::{router, sidebar, titlebar};

#[derive(Clone)]
pub struct LauncherView {}

impl LauncherView {
    pub fn new() -> Self {
        Self {}
    }

    fn content(&self) -> impl IntoElement {
        div()
            .id("content")
            .flex()
            .flex_col()
            .bg(rgb(0x1a1a1a))
            .text_color(rgb(0xe8e8e8))
            .p(px(16.0))
            .child(router::router())
    }
}

impl Render for LauncherView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0x1a1a1a))
            .flex()
            .flex_col()
            .child(titlebar::titlebar())
            .child(
                div()
                    .flex()
                    .size_full()
                    .child(sidebar::Sidebar::new())
                    .child(div().flex().flex_col().flex_1().child(self.content())),
            )
    }
}
