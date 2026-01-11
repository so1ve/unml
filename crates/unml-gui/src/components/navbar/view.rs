use gpui::prelude::*;
use gpui::*;
use gpui_component::popover::Popover;
use gpui_component::{ActiveTheme, IconName};
use gpui_markup::ui;
use gpui_router::use_location;

use super::TabItem;
use super::language_item::LanguageItem;
use super::tab_item::TabItemView;
use crate::components::icon_button::IconButton;
use crate::routes::NAV_TABS;

#[derive(IntoElement)]
pub struct NavBar {
    tabs: &'static [TabItem],
}

impl NavBar {
    pub fn new() -> Self {
        Self { tabs: NAV_TABS }
    }

    #[allow(dead_code)]
    pub fn with_tabs(mut self, tabs: &'static [TabItem]) -> Self {
        self.tabs = tabs;
        self
    }
}

impl RenderOnce for NavBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let pathname = use_location(cx).pathname.clone();
        let theme = cx.theme();

        let tab_children: Vec<AnyElement> = self
            .tabs
            .iter()
            .map(|tab| TabItemView::new(tab.clone(), pathname.clone()).into_any_element())
            .collect();

        ui! {
            div @[
                id: "navbar",
                h: px(48.0),
                w_full,
                bg: theme.tab_bar,
                border_b_1,
                border_color: theme.border,
                flex,
                items_center,
                px_4,
                gap_1
            ] {
                ..tab_children,
                div @[flex_1] {},
                Popover::new("i18n-popover") @[trigger: IconButton::new("i18n-button", IconName::Globe)] {
                    .content(|_, _, _| {
                        let current_locale = rust_i18n::locale();
                        let current: &str = &current_locale;
                        ui! {
                            div @[min_w: px(120.0), py_1] {
                                LanguageItem::new("zh-CN", current == "zh-CN"),
                                LanguageItem::new("en", current == "en")
                            }
                        }
                    })
                }
            }
        }
    }
}
