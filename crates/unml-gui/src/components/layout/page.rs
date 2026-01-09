use gpui::prelude::*;
use gpui::*;
use gpui_component::ActiveTheme;
use gpui_router::{IntoLayout, NavLink, Outlet, use_params};

use crate::components::sidebar::{SectionTitle, SidebarContent, SidebarItem, SidebarVariant};

// ============================================================================
// Page Layout - Generic layout with sidebar and content
// ============================================================================

#[derive(IntoElement, IntoLayout)]
pub struct PageLayout {
    outlet: Outlet,
    base_path: &'static str,
    content: &'static SidebarContent,
    variant: SidebarVariant,
    default_id: &'static str,
}

impl PageLayout {
    pub fn new(
        base_path: &'static str,
        content: &'static SidebarContent,
        variant: SidebarVariant,
        default_id: &'static str,
    ) -> Self {
        Self {
            outlet: Outlet::new(),
            base_path,
            content,
            variant,
            default_id,
        }
    }
}

impl RenderOnce for PageLayout {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let params = use_params(cx);
        let current_id = params
            .get("selection")
            .map(ToString::to_string)
            .unwrap_or_else(|| self.default_id.to_string());

        div()
            .flex()
            .flex_1()
            .overflow_hidden()
            .child(SidebarView::new(
                self.base_path,
                self.content,
                self.variant,
                current_id,
            ))
            .child(
                div()
                    .id("content")
                    .flex()
                    .flex_col()
                    .flex_1()
                    .bg(cx.theme().background)
                    .text_color(cx.theme().foreground)
                    .p(px(16.0))
                    .child(self.outlet),
            )
    }
}

// ============================================================================
// Sidebar View Component
// ============================================================================

#[derive(IntoElement)]
struct SidebarView {
    base_path: &'static str,
    content: &'static SidebarContent,
    variant: SidebarVariant,
    current_id: String,
}

impl SidebarView {
    fn new(
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
                            FilterItemView::new(base_path, item, active).into_any_element()
                        }
                        SidebarVariant::Navigation => {
                            NavItemView::new(base_path, item, active).into_any_element()
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

// ============================================================================
// Item Views
// ============================================================================

#[derive(IntoElement)]
struct FilterItemView {
    base_path: &'static str,
    item: &'static SidebarItem,
    active: bool,
}

impl FilterItemView {
    fn new(base_path: &'static str, item: &'static SidebarItem, active: bool) -> Self {
        Self {
            base_path,
            item,
            active,
        }
    }
}

impl RenderOnce for FilterItemView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let active = self.active;
        let path = format!("{}/{}", self.base_path, self.item.id);

        NavLink::new().to(SharedString::from(path)).child(
            div()
                .id(SharedString::from(self.item.id))
                .h(px(32.0))
                .px_2()
                .rounded(px(4.0))
                .cursor_pointer()
                .flex()
                .items_center()
                .text_color(rgb(if active { 0xe8e8e8 } else { 0xa0a0a0 }))
                .bg(rgb(if active { 0x2d2d2d } else { 0x252525 }))
                .hover(|s| s.bg(rgb(0x2d2d2d)).text_color(rgb(0xe8e8e8)))
                .child(SharedString::from(self.item.label)),
        )
    }
}

#[derive(IntoElement)]
struct NavItemView {
    base_path: &'static str,
    item: &'static SidebarItem,
    active: bool,
}

impl NavItemView {
    fn new(base_path: &'static str, item: &'static SidebarItem, active: bool) -> Self {
        Self {
            base_path,
            item,
            active,
        }
    }
}

impl RenderOnce for NavItemView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let active = self.active;
        let path = format!("{}/{}", self.base_path, self.item.id);

        let item_div = div()
            .id(SharedString::from(self.item.id))
            .h(px(40.0))
            .px_3()
            .rounded(px(6.0))
            .cursor_pointer()
            .flex()
            .items_center()
            .text_color(rgb(if active { 0xe8e8e8 } else { 0xa0a0a0 }))
            .bg(rgb(if active { 0x2d2d2d } else { 0x252525 }))
            .hover(|s| s.bg(rgb(0x2d2d2d)).text_color(rgb(0xe8e8e8)))
            .child(SharedString::from(self.item.label));

        let item_div = if active {
            item_div.border_l_2().border_color(rgb(0x3b82f6))
        } else {
            item_div
        };

        NavLink::new().to(SharedString::from(path)).child(item_div)
    }
}
