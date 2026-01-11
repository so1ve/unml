use gpui::prelude::*;
use gpui::*;
use gpui_component::{ActiveTheme, Icon, IconName};
use gpui_markup::ui;
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
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let label = match self.locale {
            "zh-CN" => t!("lang.zh-CN"),
            "en" => t!("lang.en"),
            _ => t!("lang.en"),
        }
        .to_string();

        let locale = self.locale;
        let theme = cx.theme();
        let text_color = if self.selected {
            theme.foreground
        } else {
            theme.muted_foreground
        };
        let item_id = SharedString::from(self.locale);

        ui! {
            div @[
                id: item_id,
                h: px(32.0),
                px_3,
                cursor_pointer,
                flex,
                items_center,
                justify_between,
                text_color: text_color,
                hover: |s| s.bg(theme.list_hover).text_color(theme.foreground),
                on_click: move |_, _, cx| {
                    rust_i18n::set_locale(locale);
                    cx.refresh_windows();
                }
            ] {
                label,
                .when(self.selected, |s| {
                    s.child(
                        ui! {
                            Icon::new(IconName::Check) @[size_4, text_color: theme.primary] {}
                        }
                    )
                })
            }
        }
    }
}
