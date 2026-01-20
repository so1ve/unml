use gpui::*;
use gpui_component::ActiveTheme;
use gpui_markup::ui;
use gpui_router::use_params;
use rust_i18n::t;
use unml_macros::PageRoute;

#[derive(PageRoute)]
#[route(path = "/downloads", label = "nav.downloads", icon = ArrowDown)]
#[sidebar(
    variant = Filter,
    section {
        InProgress => "downloads.in_progress",
        Completed => "downloads.completed",
        Failed => "downloads.failed",
    }
)]
pub struct DownloadsPage;

impl DownloadsPage {
    pub fn view(_window: &mut Window, cx: &mut App) -> impl IntoElement {
        let params = use_params(cx);
        let selection = params
            .get("subroute")
            .map(|s| s.as_str())
            .unwrap_or("Downloading");

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
