use gpui::prelude::*;
use gpui::*;
use gpui_markup::ui;
use rust_i18n::t;

#[derive(IntoElement)]
pub struct PageContent {
    title: Option<String>,
    children: AnyElement,
}

impl PageContent {
    pub fn new(title: Option<&'static str>, children: AnyElement) -> Self {
        let title = title.map(|key| t!(key).to_string());

        Self { title, children }
    }
}

impl RenderOnce for PageContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        ui! {
            div @[flex, flex_col, flex_1, gap: px(10.0), overflow_hidden] {
                ..self.title.map(|title| ui! {
                    div @[text_size: px(20.0)] {
                        title
                    }
                }),

                self.children
            }
        }
    }
}
