use gpui::*;
use gpui_component::ActiveTheme;
use gpui_markup::ui;
use rust_i18n::t;
use unml_macros::SubRoute;

use crate::routing::PageView;

#[derive(SubRoute)]
#[subroute(id = "Browse")]
#[layout(title = "mods.browse")]
pub struct BrowseModsPage;

impl PageView for BrowseModsPage {
    fn view(_window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        ui! {
            div @[text_color: theme.muted_foreground] {
                t!("mods.browse_placeholder").to_string()
            }
        }
    }
}
