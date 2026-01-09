use gpui::prelude::*;
use gpui::*;
use gpui_component::ActiveTheme;
use gpui_router::{IntoLayout, Outlet, use_params};

use crate::components::sidebar::{SidebarContent, SidebarVariant, SidebarView};

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
