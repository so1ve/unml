use gpui::prelude::*;
use gpui::*;

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
// Section Title Component
// ============================================================================

#[derive(IntoElement)]
pub struct SectionTitle {
    title: &'static str,
}

impl SectionTitle {
    pub fn new(title: &'static str) -> Self {
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
