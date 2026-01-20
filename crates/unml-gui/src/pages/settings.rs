mod java;

use gpui::*;
use gpui_markup::ui;
use java::JavaSettingsPage;
use rust_i18n::t;
use unml_macros::PageRoute;

use crate::routing::PageView;

#[derive(PageRoute)]
#[route(id = "settings", label = "nav.settings", icon = Settings)]
#[sidebar(
    variant = Navigation,
    section {
        java => "settings.java",
    }
)]
#[children(JavaSettingsPage)]
pub struct SettingsPage;

impl PageView for SettingsPage {
    fn view(_window: &mut Window, _cx: &mut App) -> impl IntoElement {
        ui! {
            div @[flex, items_center, justify_center, size_full, text_lg] {
                t!("settings.select_category").to_string()
            }
        }
    }
}
