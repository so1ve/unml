use gpui::*;
use gpui_router::use_params;

use crate::components::sidebar::{SidebarContent, SidebarItem, SidebarSection};

// ============================================================================
// Page Path & Selection
// ============================================================================

pub const PATH: &str = "/mods";

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum Selection {
    #[default]
    Installed,
    Browse,
    Fabric,
    Forge,
    Quilt,
}

impl Selection {
    pub const fn id(&self) -> &'static str {
        match self {
            Self::Installed => "installed",
            Self::Browse => "browse",
            Self::Fabric => "fabric",
            Self::Forge => "forge",
            Self::Quilt => "quilt",
        }
    }

    pub fn from_id(id: &str) -> Self {
        match id {
            "installed" => Self::Installed,
            "browse" => Self::Browse,
            "fabric" => Self::Fabric,
            "forge" => Self::Forge,
            "quilt" => Self::Quilt,
            _ => Self::default(),
        }
    }
}

// ============================================================================
// Sidebar Content
// ============================================================================

pub const DEFAULT_ID: &str = Selection::Installed.id();

const VIEW_ITEMS: &[SidebarItem] = &[
    SidebarItem::new("installed", "已安装"),
    SidebarItem::new("browse", "浏览"),
];

const FILTER_ITEMS: &[SidebarItem] = &[
    SidebarItem::new("fabric", "Fabric"),
    SidebarItem::new("forge", "Forge"),
    SidebarItem::new("quilt", "Quilt"),
];

pub static SIDEBAR: SidebarContent = SidebarContent::new(&[
    SidebarSection::new(VIEW_ITEMS).with_title("视图"),
    SidebarSection::new(FILTER_ITEMS).with_title("筛选"),
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
            Selection::Installed => "已安装的 Mod 列表",
            Selection::Browse => "浏览在线 Mod",
            Selection::Fabric => "Fabric Mod",
            Selection::Forge => "Forge Mod",
            Selection::Quilt => "Quilt Mod",
        };

        div()
            .flex()
            .flex_col()
            .gap(px(10.0))
            .child(
                div()
                    .text_size(px(20.0))
                    .child(SharedString::from("Mod 管理")),
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
