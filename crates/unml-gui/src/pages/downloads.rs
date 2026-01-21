use gpui::*;
use gpui_component::ActiveTheme;
use gpui_markup::ui;
use gpui_router::use_params;
use rust_i18n::t;
use unml_macros::PageRoute;

use crate::routing::PageView;

#[derive(PageRoute)]
#[route(id = "downloads", label = "nav.downloads", icon = ArrowDown)]
#[layout(title = "downloads.title")]
#[sidebar(
    variant = Filter,
    section {
        InProgress => "downloads.in_progress",
        Completed => "downloads.completed",
        Failed => "downloads.failed",
    }
)]
pub struct DownloadsPage;

impl PageView for DownloadsPage {
    fn view(_window: &mut Window, cx: &mut App) -> impl IntoElement {
        let params = use_params(cx);
        let selection = params
            .get("subroute")
            .map(|s| s.as_str())
            .unwrap_or("Downloading");

        let theme = cx.theme();

        ui! {
            div @[text_color: theme.muted_foreground] {
                format!("Selection: {}", selection)
            }
        }
    }
}
