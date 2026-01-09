use gpui::*;
use gpui_component::ActiveTheme;

use crate::components::{navbar, router, sidebar, titlebar};

#[derive(Clone)]
pub struct LauncherView;

impl LauncherView {
    pub fn new() -> Self {
        Self
    }

    fn content(&self, cx: &App) -> impl IntoElement {
        div()
            .id("content")
            .flex()
            .flex_col()
            .flex_1()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .p(px(16.0))
            .child(router::router())
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
            // Main layout: Sidebar + Content
            .child(
                div()
                    .flex()
                    .flex_1()
                    .overflow_hidden()
                    .child(sidebar::ContextSidebar::new())
                    .child(self.content(cx)),
            )
    }
}
