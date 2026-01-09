use gpui::prelude::*;
use gpui::*;
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
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .text_xs()
            .font_weight(FontWeight::MEDIUM)
            .text_color(rgb(0x888888))
            .mb_1()
            .child(t!(self.title).to_string().to_uppercase())
    }
}
