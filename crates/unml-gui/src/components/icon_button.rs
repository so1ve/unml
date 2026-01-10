//! Custom icon button component.

use gpui::prelude::*;
use gpui::*;
use gpui_component::{ActiveTheme, Icon, IconName, Selectable};
use gpui_markup::ui;

/// A simple icon button with hover and active states.
#[derive(IntoElement)]
pub struct IconButton {
    id: ElementId,
    icon: IconName,
    size: Pixels,
}

impl IconButton {
    pub fn new(id: impl Into<ElementId>, icon: IconName) -> Self {
        Self {
            id: id.into(),
            icon,
            size: px(36.0),
        }
    }

    #[allow(dead_code)]
    pub fn size(mut self, size: Pixels) -> Self {
        self.size = size;
        self
    }
}

impl Selectable for IconButton {
    fn selected(self, _selected: bool) -> Self {
        self
    }

    fn is_selected(&self) -> bool {
        true
    }
}

impl RenderOnce for IconButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        ui! {
            <div
                id={self.id}
                size={self.size}
                rounded={px(6.0)}
                cursor_pointer
                flex
                items_center
                justify_center
                text_color={theme.muted_foreground}
                hover={|s| s.bg(theme.secondary).text_color(theme.foreground)}
                active={|s| s.bg(theme.secondary_hover)}
            >
                <{Icon::new(self.icon).size_4()} />
            </div>
        }
    }
}
