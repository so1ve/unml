use gpui::*;
use gpui_component::ActiveTheme;
use gpui_markup::ui;
use gpui_router::use_params;
use rust_i18n::t;

unml_macros::define_sidebar! {
    variant: Navigation,

    section {
        General => "settings.general",
        Java => "settings.java",
        Game => "settings.game",
        Download => "settings.download",
        About => "settings.about",
    }
}

#[derive(IntoElement)]
pub struct Page;

impl RenderOnce for Page {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let params = use_params(cx);
        let selection = params
            .get("selection")
            .map(|s| Selection::from_id(s))
            .unwrap_or_default();

        let (title, content) = match selection {
            Selection::General => (t!("settings.general_title"), t!("settings.general_desc")),
            Selection::Java => (t!("settings.java_title"), t!("settings.java_desc")),
            Selection::Game => (t!("settings.game_title"), t!("settings.game_desc")),
            Selection::Download => (t!("settings.download_title"), t!("settings.download_desc")),
            Selection::About => (t!("settings.about_title"), t!("settings.about_desc")),
        };

        let theme = cx.theme();

        ui! {
            div @[flex, flex_col, gap: px(10.0)] {
                div @[text_size: px(20.0)] {
                    title.to_string()
                },
                div @[text_color: theme.muted_foreground] {
                    content.to_string()
                }
            }
        }
    }
}

pub fn page() -> Page {
    Page
}
