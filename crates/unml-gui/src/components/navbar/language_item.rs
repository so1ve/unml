use gpui::prelude::*;
use gpui::*;
use gpui_component::{Icon, IconName};
use rust_i18n::t;

#[derive(IntoElement)]
pub struct LanguageItem {
    locale: &'static str,
    selected: bool,
}

impl LanguageItem {
    pub fn new(locale: &'static str, selected: bool) -> Self {
        Self { locale, selected }
    }
}

impl RenderOnce for LanguageItem {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let label = match self.locale {
            "zh-CN" => t!("lang.zh-CN"),
            "en" => t!("lang.en"),
            _ => t!("lang.en"),
        }
        .to_string();

        let locale = self.locale;

        div()
            .id(SharedString::from(self.locale))
            .h(px(32.0))
            .px_3()
            .cursor_pointer()
            .flex()
            .items_center()
            .justify_between()
            .text_color(rgb(if self.selected { 0xe8e8e8 } else { 0xa0a0a0 }))
            .hover(|s| s.bg(rgb(0x2d2d2d)).text_color(rgb(0xe8e8e8)))
            .on_click(move |_, _, cx| {
                rust_i18n::set_locale(locale);
                cx.refresh_windows();
            })
            .child(label)
            .when(self.selected, |s| {
                s.child(
                    Icon::new(IconName::Check)
                        .size_4()
                        .text_color(rgb(0x3b82f6)),
                )
            })
    }
}
