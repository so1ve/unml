use gpui::prelude::*;
use gpui::*;
use gpui_component::scroll::ScrollableElement;
use gpui_markup::ui;
use rust_i18n::t;
use unml_macros::PageRoute;

use crate::components::instance_card::InstanceCard;

#[derive(PageRoute)]
#[route(id = "", label = "nav.home", icon = LayoutDashboard, home)]
pub struct HomePage;

impl HomePage {
    pub fn view(_window: &mut Window, _cx: &mut App) -> impl IntoElement {
        ui! {
            div @[size_full, overflow_y_scrollbar] {
                div @[flex, flex_col, gap: px(32.0)] {
                    div @[flex, flex_col, gap: px(16.0)] {
                        div @[text_size: px(18.0), font_weight: FontWeight::BOLD] {
                            t!("home.favorites").to_string()
                        },
                        div @[flex, flex_row, flex_wrap, gap: px(16.0)] {
                            InstanceCard::new("Survival World", "1.20.4", "Fabric", rgb(0x4caf50))
                        }
                    },
                    div @[flex, flex_col, gap: px(16.0)] {
                        div @[text_size: px(18.0), font_weight: FontWeight::BOLD] {
                            t!("home.recent").to_string()
                        },
                        div @[flex, flex_row, flex_wrap, gap: px(16.0)] {
                            InstanceCard::new("Test Server", "1.19.2", "Forge", rgb(0x2196f3)),
                            InstanceCard::new("Modpack 1", "1.18.2", "Quilt", rgb(0xff9800)),
                            InstanceCard::new("Vanilla", "1.21", "Vanilla", rgb(0x9e9e9e))
                        }
                    }
                }
            }
        }
    }
}
