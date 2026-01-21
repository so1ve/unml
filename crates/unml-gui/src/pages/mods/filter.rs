use gpui::*;
use gpui_component::ActiveTheme;
use gpui_markup::ui;
use rust_i18n::t;
use unml_macros::SubRoute;

use crate::routing::PageView;

#[derive(SubRoute)]
#[subroute(id = "Fabric")]
#[layout(title = "mods.fabric")]
pub struct FabricModsPage;

impl PageView for FabricModsPage {
    fn view(_window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        ui! {
            div @[text_color: theme.muted_foreground] {
                t!("mods.fabric_placeholder").to_string()
            }
        }
    }
}

#[derive(SubRoute)]
#[subroute(id = "Forge")]
#[layout(title = "mods.forge")]
pub struct ForgeModsPage;

impl PageView for ForgeModsPage {
    fn view(_window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        ui! {
            div @[text_color: theme.muted_foreground] {
                t!("mods.forge_placeholder").to_string()
            }
        }
    }
}

#[derive(SubRoute)]
#[subroute(id = "Quilt")]
#[layout(title = "mods.quilt")]
pub struct QuiltModsPage;

impl PageView for QuiltModsPage {
    fn view(_window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        ui! {
            div @[text_color: theme.muted_foreground] {
                t!("mods.quilt_placeholder").to_string()
            }
        }
    }
}
