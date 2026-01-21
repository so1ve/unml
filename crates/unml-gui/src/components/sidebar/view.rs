use gpui::prelude::*;
use gpui::*;
use gpui_component::ActiveTheme;
use gpui_markup::ui;

use super::{FilterItem, NavItem, SectionTitle, SidebarContent, SidebarVariant};

#[derive(IntoElement)]
pub struct SidebarView {
    route_id: &'static str,
    content: &'static SidebarContent,
    variant: SidebarVariant,
    current_id: String,
}

impl SidebarView {
    pub fn new(
        route_id: &'static str,
        content: &'static SidebarContent,
        variant: SidebarVariant,
        current_id: String,
    ) -> Self {
        Self {
            route_id,
            content,
            variant,
            current_id,
        }
    }
}

impl RenderOnce for SidebarView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let sections = self.content.sections;
        let section_count = sections.len();
        let variant = self.variant;
        let route_id = self.route_id;
        let current_id = self.current_id;
        let theme = cx.theme();

        let children: Vec<AnyElement> = sections
            .iter()
            .enumerate()
            .flat_map(|(idx, section)| {
                let mut elements: Vec<AnyElement> = Vec::new();

                if let Some(title) = section.title {
                    elements.push(SectionTitle::new(title).into_any_element());
                }

                for item in section.items {
                    let active = item.id == current_id;
                    let element = match variant {
                        SidebarVariant::Filter => {
                            FilterItem::new(route_id, item, active).into_any_element()
                        }
                        SidebarVariant::Navigation => {
                            NavItem::new(route_id, item, active).into_any_element()
                        }
                    };
                    elements.push(element);
                }

                if idx < section_count - 1 {
                    elements.push(
                        ui! {
                            div @[h: px(1.0), bg: theme.border, my_2] {}
                        }
                        .into_any_element(),
                    );
                }

                elements
            })
            .collect();

        ui! {
            div @[
                id: "sidebar",
                w: px(240.0),
                h_full,
                bg: theme.sidebar,
                border_r_1,
                border_color: theme.border,
                flex,
                flex_col,
                p_4,
                gap_3
            ] {
                ..children
            }
        }
    }
}
