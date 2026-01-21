use gpui::*;
use gpui_component::ActiveTheme;
use gpui_markup::ui;
use gpui_router::use_params;
use rust_i18n::t;
use unml_macros::PageRoute;

use crate::routing::PageView;

#[derive(PageRoute)]
#[route(id = "mods", label = "nav.mods", icon = Star)]
#[layout(title = "mods.title")]
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

impl PageView for ModsPage {
    fn view(_window: &mut Window, cx: &mut App) -> impl IntoElement {
        let params = use_params(cx);
        let selection = params
            .get("subroute")
            .map(|s| s.as_str())
            .unwrap_or("Installed");

        let theme = cx.theme();

        ui! {
            div @[text_color: theme.muted_foreground] {
                format!("Selection: {}", selection)
            }
        }
    }
}
