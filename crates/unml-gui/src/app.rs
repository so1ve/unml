use gpui::*;
use gpui_component::ActiveTheme;
use unml_core::Account;

use crate::components::{router, sidebar, titlebar};

#[derive(Clone)]
pub struct LauncherView {
    account: Option<Account>,
}

impl LauncherView {
    pub fn new() -> Self {
        Self { account: None }
    }

    fn content(&self, cx: &App) -> impl IntoElement {
        div()
            .id("content")
            .flex()
            .flex_col()
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
            .child(titlebar::titlebar(self.account.clone()))
            .child(
                div()
                    .flex()
                    .size_full()
                    .child(sidebar::Sidebar::new())
                    .child(div().flex().flex_col().flex_1().child(self.content(cx))),
            )
    }
}
