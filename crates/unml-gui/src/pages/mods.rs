use gpui::*;
use gpui_component::ActiveTheme;
use gpui_markup::ui;
use gpui_router::use_params;
use rust_i18n::t;
use unml_macros::PageRoute;

#[derive(PageRoute)]
#[route(id = "mods", label = "nav.mods", icon = Star)]
#[sidebar(
    variant = Filter,
    section "mods.view" {
        Installed => "mods.installed",
        Browse => "mods.browse",
    },
    section "mods.filter" {
        Fabric => "mods.fabric",
        Forge => "mods.forge",
        Quilt => "mods.quilt",
    }
)]
pub struct ModsPage;

impl ModsPage {
    pub fn view(_window: &mut Window, cx: &mut App) -> impl IntoElement {
        let params = use_params(cx);
        let selection = params
            .get("subroute")
            .map(|s| s.as_str())
            .unwrap_or("Installed");

        let theme = cx.theme();

        ui! {
            div @[flex, flex_col, gap: px(10.0)] {
                div @[text_size: px(20.0)] {
                    t!("mods.title").to_string()
                },
                div @[text_color: theme.muted_foreground] {
                    format!("Selection: {}", selection)
                }
            }
        }
    }
}
