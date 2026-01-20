use gpui::*;
use gpui_component::ActiveTheme;
use gpui_markup::ui;
use rust_i18n::t;
use unml_macros::SubRoute;

use crate::routing::PageView;

#[derive(SubRoute)]
#[subroute(id = "java")]
pub struct JavaSettingsPage;

impl PageView for JavaSettingsPage {
    fn view(_window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        ui! {
            div @[flex, flex_col, gap: px(16.)] {
                div @[text_xl, font_weight: FontWeight::SEMIBOLD] {
                    t!("settings.java_title").to_string()
                },
                div @[text_color: theme.muted_foreground] {
                    t!("settings.java_desc").to_string()
                },
                div @[flex, items_center, justify_center, h: px(200.), text_color: theme.muted_foreground] {
                    "Java settings placeholder - to be implemented"
                }
            }
        }
    }
}
