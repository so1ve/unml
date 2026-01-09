use gpui::prelude::*;
use gpui::*;
use gpui_component::ActiveTheme;
use gpui_router::{NavLink, use_location, use_params};

use crate::pages;

// ============================================================================
// Sidebar Item Types
// ============================================================================

/// A single item in a sidebar section
#[derive(Clone, Copy)]
pub struct SidebarItem {
    pub id: &'static str,
    pub label: &'static str,
}

impl SidebarItem {
    pub const fn new(id: &'static str, label: &'static str) -> Self {
        Self { id, label }
    }
}

/// A section in the sidebar with a title and items
#[derive(Clone, Copy)]
pub struct SidebarSection {
    pub title: Option<&'static str>,
    pub items: &'static [SidebarItem],
}

impl SidebarSection {
    pub const fn new(items: &'static [SidebarItem]) -> Self {
        Self { title: None, items }
    }

    pub const fn with_title(mut self, title: &'static str) -> Self {
        self.title = Some(title);
        self
    }
}

/// Sidebar content definition - sections separated by dividers
#[derive(Clone, Copy)]
pub struct SidebarContent {
    pub sections: &'static [SidebarSection],
}

impl SidebarContent {
    pub const fn new(sections: &'static [SidebarSection]) -> Self {
        Self { sections }
    }
}

/// Sidebar variant determines the rendering style
#[derive(Clone, Copy)]
pub enum SidebarVariant {
    /// Standard filter style (small items)
    Filter,
    /// Navigation style (larger items with left border indicator)
    Navigation,
}

// ============================================================================
// Context Sidebar Component
// ============================================================================

/// Context-aware sidebar that shows different content based on current page
#[derive(IntoElement)]
pub struct ContextSidebar;

impl ContextSidebar {
    pub fn new() -> Self {
        Self
    }
}

impl RenderOnce for ContextSidebar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let location = use_location(cx);
        let pathname = location.pathname.clone();

        // Extract base path (e.g., "/versions/release" -> "/versions")
        let base_path = pathname.split('/').take(2).collect::<Vec<_>>().join("/");
        let base_path = if base_path.is_empty() {
            "/"
        } else {
            &base_path
        };

        div()
            .id("context-sidebar")
            .w(px(240.0))
            .h_full()
            .bg(rgb(0x252525))
            .border_r_1()
            .border_color(cx.theme().border)
            .flex()
            .flex_col()
            .child(match base_path {
                "/" => pages::home::Sidebar::new().into_any_element(),
                "/versions" => SidebarView::new(
                    pages::versions::PATH,
                    &pages::versions::SIDEBAR,
                    SidebarVariant::Filter,
                    pages::versions::DEFAULT_ID,
                )
                .into_any_element(),
                "/mods" => SidebarView::new(
                    pages::mods::PATH,
                    &pages::mods::SIDEBAR,
                    SidebarVariant::Filter,
                    pages::mods::DEFAULT_ID,
                )
                .into_any_element(),
                "/downloads" => SidebarView::new(
                    pages::downloads::PATH,
                    &pages::downloads::SIDEBAR,
                    SidebarVariant::Filter,
                    pages::downloads::DEFAULT_ID,
                )
                .into_any_element(),
                "/settings" => SidebarView::new(
                    pages::settings::PATH,
                    &pages::settings::SIDEBAR,
                    SidebarVariant::Navigation,
                    pages::settings::DEFAULT_ID,
                )
                .into_any_element(),
                _ => pages::home::Sidebar::new().into_any_element(),
            })
    }
}

// ============================================================================
// Sidebar View Component
// ============================================================================

/// Generic sidebar view that renders sections with items
#[derive(IntoElement)]
pub struct SidebarView {
    base_path: &'static str,
    content: &'static SidebarContent,
    variant: SidebarVariant,
    default_id: &'static str,
}

impl SidebarView {
    pub fn new(
        base_path: &'static str,
        content: &'static SidebarContent,
        variant: SidebarVariant,
        default_id: &'static str,
    ) -> Self {
        Self {
            base_path,
            content,
            variant,
            default_id,
        }
    }
}

impl RenderOnce for SidebarView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let sections = self.content.sections;
        let section_count = sections.len();
        let variant = self.variant;
        let base_path = self.base_path;

        // Get current selection from route params, default to page's default
        let params = use_params(cx);
        let current_id = params
            .get("selection")
            .map(|s| s.as_str())
            .unwrap_or(self.default_id);

        div()
            .flex()
            .flex_col()
            .p_4()
            .gap_3()
            .children(sections.iter().enumerate().flat_map(|(idx, section)| {
                let mut elements: Vec<AnyElement> = Vec::new();

                // Add section title if present
                if let Some(title) = section.title {
                    elements.push(SectionTitle::new(title).into_any_element());
                }

                // Add items based on variant
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

                // Add divider between sections (not after the last one)
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
// Helper Components
// ============================================================================

/// Section title component
#[derive(IntoElement)]
struct SectionTitle {
    title: &'static str,
}

impl SectionTitle {
    fn new(title: &'static str) -> Self {
        Self { title }
    }
}

impl RenderOnce for SectionTitle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .text_xs()
            .font_weight(FontWeight::MEDIUM)
            .text_color(rgb(0x888888))
            .mb_1()
            .child(SharedString::from(self.title.to_uppercase()))
    }
}

/// Filter item view (compact style) - uses NavLink for navigation
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

/// Navigation item view with left border indicator - uses NavLink for
/// navigation
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

        NavLink::new().to(SharedString::from(path)).child(
            div()
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
                .when(active, |s| s.border_l_2().border_color(rgb(0x3b82f6)))
                .child(SharedString::from(self.item.label)),
        )
    }
}
