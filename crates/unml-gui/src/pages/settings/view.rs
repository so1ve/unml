use gpui::*;
use gpui_component::ActiveTheme;
use gpui_markup::ui;
use gpui_router::use_params;
use rust_i18n::t;

use super::Selection;
use super::java::JavaSettingsGlobal;

#[derive(IntoElement)]
pub struct Page;

impl RenderOnce for Page {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let params = use_params(cx);
        let selection = params
            .get("selection")
            .map(|s| Selection::from_id(s))
            .unwrap_or_default();

        let theme = cx.theme();

        match selection {
            Selection::Java => {
                let java_view = JavaSettingsGlobal::get_or_create(window, cx);

                ui! {
                    div @[size_full, min_w_0, overflow_hidden] {
                        java_view
                    }
                }
            }
            Selection::General => render_placeholder(
                t!("settings.general_title"),
                t!("settings.general_desc"),
                theme,
            ),
            Selection::Game => {
                render_placeholder(t!("settings.game_title"), t!("settings.game_desc"), theme)
            }
            Selection::Download => render_placeholder(
                t!("settings.download_title"),
                t!("settings.download_desc"),
                theme,
            ),
            Selection::About => {
                render_placeholder(t!("settings.about_title"), t!("settings.about_desc"), theme)
            }
        }
        .into_any_element()
    }
}

fn render_placeholder(
    title: std::borrow::Cow<'_, str>,
    content: std::borrow::Cow<'_, str>,
    theme: &gpui_component::Theme,
) -> Div {
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

pub fn page() -> Page {
    Page
}
