use gpui::prelude::*;
use gpui::*;
use gpui_component::ActiveTheme;
use gpui_markup::ui;
use rust_i18n::t;

#[derive(IntoElement)]
pub struct SectionTitle {
    title: &'static str,
}

impl SectionTitle {
    pub fn new(title: &'static str) -> Self {
        Self { title }
    }
}

impl RenderOnce for SectionTitle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let title = t!(self.title).to_string().to_uppercase();

        ui! {
            div @[
                text_xs,
                font_weight: FontWeight::MEDIUM,
                text_color: cx.theme().muted_foreground,
                mb_1
            ] {
                title
            }
        }
    }
}
