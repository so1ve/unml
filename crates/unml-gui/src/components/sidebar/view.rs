use gpui::prelude::*;
use gpui::*;
use gpui_component::ActiveTheme;

use super::{FilterItem, NavItem, SectionTitle, SidebarContent, SidebarVariant};

#[derive(IntoElement)]
pub struct SidebarView {
    base_path: &'static str,
    content: &'static SidebarContent,
    variant: SidebarVariant,
    current_id: String,
}

impl SidebarView {
    pub fn new(
        base_path: &'static str,
        content: &'static SidebarContent,
        variant: SidebarVariant,
        current_id: String,
    ) -> Self {
        Self {
            base_path,
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
        let base_path = self.base_path;
        let current_id = self.current_id;

        div()
            .id("sidebar")
            .w(px(240.0))
            .h_full()
            .bg(rgb(0x252525))
            .border_r_1()
            .border_color(cx.theme().border)
            .flex()
            .flex_col()
            .p_4()
            .gap_3()
            .children(sections.iter().enumerate().flat_map(|(idx, section)| {
                let mut elements: Vec<AnyElement> = Vec::new();

                if let Some(title) = section.title {
                    elements.push(SectionTitle::new(title).into_any_element());
                }

                for item in section.items {
                    let active = item.id == current_id;
                    let element = match variant {
                        SidebarVariant::Filter => {
                            FilterItem::new(base_path, item, active).into_any_element()
                        }
                        SidebarVariant::Navigation => {
                            NavItem::new(base_path, item, active).into_any_element()
                        }
                    };
                    elements.push(element);
                }

                if idx < section_count - 1 {
                    elements.push(
                        div()
                            .h(px(1.0))
                            .bg(cx.theme().border)
                            .my_2()
                            .into_any_element(),
                    );
                }

                elements
            }))
    }
}
