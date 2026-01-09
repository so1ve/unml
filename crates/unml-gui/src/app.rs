use gpui::*;
use gpui_component::ActiveTheme;

use crate::components::{navbar, router, titlebar};

#[derive(Clone)]
pub struct LauncherView;

impl LauncherView {
    pub fn new() -> Self {
        Self
    }
}

impl Render for LauncherView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(cx.theme().background)
            .flex()
            .flex_col()
            // TitleBar (40px)
            .child(titlebar::titlebar())
            // NavBar (48px)
            .child(navbar::NavBar::new())
            // Main content (router handles layout with sidebar)
            .child(
                div()
                    .flex()
                    .flex_1()
                    .overflow_hidden()
                    .bg(cx.theme().background)
                    .text_color(cx.theme().foreground)
                    .child(router::router()),
            )
    }
}
