use gpui::*;
use gpui_router::use_params;
use unml_macros::Selection;

use crate::components::sidebar::{SidebarContent, SidebarItem, SidebarSection};

// ============================================================================
// Page Path & Selection
// ============================================================================

pub const PATH: &str = "/versions";

#[derive(Clone, Copy, PartialEq, Eq, Selection)]
pub enum Selection {
    #[default]
    #[id = "release"]
    Release,
    #[id = "snapshot"]
    Snapshot,
    #[id = "old"]
    Old,
    #[id = "installed"]
    Installed,
}

pub const DEFAULT_ID: &str = Selection::default().id();

// ============================================================================
// Sidebar Content
// ============================================================================

const FILTER_ITEMS: &[SidebarItem] = &[
    SidebarItem::new("release", "正式版"),
    SidebarItem::new("snapshot", "快照版"),
    SidebarItem::new("old", "远古版本"),
];

const OPTIONS_ITEMS: &[SidebarItem] = &[SidebarItem::new("installed", "仅已安装")];

pub static SIDEBAR: SidebarContent = SidebarContent::new(&[
    SidebarSection::new(FILTER_ITEMS).with_title("筛选"),
    SidebarSection::new(OPTIONS_ITEMS),
]);

// ============================================================================
// Page Content
// ============================================================================

#[derive(IntoElement)]
pub struct Page;

impl RenderOnce for Page {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let params = use_params(cx);
        let selection = params
            .get("selection")
            .map(|s| Selection::from_id(s))
            .unwrap_or_default();

        let content = match selection {
            Selection::Release => "正式版版本列表",
            Selection::Snapshot => "快照版版本列表",
            Selection::Old => "远古版本列表",
            Selection::Installed => "已安装的版本",
        };

        div()
            .flex()
            .flex_col()
            .gap(px(10.0))
            .child(
                div()
                    .text_size(px(20.0))
                    .child(SharedString::from("版本管理")),
            )
            .child(
                div()
                    .text_color(rgb(0xa0a0a0))
                    .child(SharedString::from(content)),
            )
    }
}

pub fn page() -> Page {
    Page
}
