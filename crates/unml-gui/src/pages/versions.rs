use gpui::*;
use gpui_component::ActiveTheme;
use gpui_markup::ui;
use gpui_router::use_params;
use rust_i18n::t;
use unml_macros::PageRoute;

#[derive(PageRoute)]
#[route(id = "versions", label = "nav.versions", icon = Folder)]
#[sidebar(
    variant = Filter,
    section "versions.filter" {
        Release => "versions.release",
        Snapshot => "versions.snapshot",
        Old => "versions.old",
    },
    section {
        Installed => "versions.installed_only",
    }
)]
pub struct VersionsPage;

impl VersionsPage {
    pub fn view(_window: &mut Window, cx: &mut App) -> impl IntoElement {
        let params = use_params(cx);
        let selection = params
            .get("subroute")
            .map(|s| s.as_str())
            .unwrap_or("Release");

        let theme = cx.theme();

        ui! {
            div @[flex, flex_col, gap: px(10.0)] {
                div @[text_size: px(20.0)] {
                    t!("versions.title").to_string()
                },
                div @[text_color: theme.muted_foreground] {
                    format!("Selection: {}", selection)
                }
            }
        }
    }
}
