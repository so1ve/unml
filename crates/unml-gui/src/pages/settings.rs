use gpui::*;
use gpui_router::use_params;

use crate::components::sidebar::{SidebarContent, SidebarItem, SidebarSection};

// ============================================================================
// Page Path & Selection
// ============================================================================

pub const PATH: &str = "/settings";

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum Selection {
    #[default]
    General,
    Java,
    Game,
    Download,
    About,
}

impl Selection {
    pub const fn id(&self) -> &'static str {
        match self {
            Self::General => "general",
            Self::Java => "java",
            Self::Game => "game",
            Self::Download => "download",
            Self::About => "about",
        }
    }

    pub fn from_id(id: &str) -> Self {
        match id {
            "general" => Self::General,
            "java" => Self::Java,
            "game" => Self::Game,
            "download" => Self::Download,
            "about" => Self::About,
            _ => Self::default(),
        }
    }
}

// ============================================================================
// Sidebar Content
// ============================================================================

pub const DEFAULT_ID: &str = Selection::General.id();

const NAV_ITEMS: &[SidebarItem] = &[
    SidebarItem::new("general", "通用"),
    SidebarItem::new("java", "Java"),
    SidebarItem::new("game", "游戏"),
    SidebarItem::new("download", "下载"),
    SidebarItem::new("about", "关于"),
];

pub static SIDEBAR: SidebarContent = SidebarContent::new(&[SidebarSection::new(NAV_ITEMS)]);

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

        let (title, content) = match selection {
            Selection::General => ("通用设置", "语言、主题等通用设置"),
            Selection::Java => ("Java 设置", "Java 路径和版本管理"),
            Selection::Game => ("游戏设置", "默认内存、窗口大小等"),
            Selection::Download => ("下载设置", "并发数、镜像源选择"),
            Selection::About => ("关于", "版本信息和更新"),
        };

        div()
            .flex()
            .flex_col()
            .gap(px(10.0))
            .child(div().text_size(px(20.0)).child(SharedString::from(title)))
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
