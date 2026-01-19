use gpui::*;
use gpui_component::ActiveTheme;
use gpui_markup::ui;
use gpui_router::use_params;
use rust_i18n::t;

unml_macros::define_sidebar! {
    variant: Filter,

    section "downloads.status" {
        All => "downloads.all",
        InProgress => "downloads.in_progress",
        Completed => "downloads.completed",
        Failed => "downloads.failed",
    }
}

#[derive(IntoElement)]
pub struct Page;

impl RenderOnce for Page {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let params = use_params(cx);
        let selection = params
            .get("selection")
            .map(|s| s.as_str())
            .unwrap_or(DEFAULT_ID.unwrap_or("All"));

        let theme = cx.theme();

        ui! {
            div @[flex, flex_col, gap: px(10.0)] {
                div @[text_size: px(20.0)] {
                    t!("downloads.title").to_string()
                },
                div @[text_color: theme.muted_foreground] {
                    format!("Selection: {}", selection)
                }
            }
        }
    }
}

pub fn page() -> Page {
    Page
}
