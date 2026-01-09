use gpui::*;
use gpui_router::use_params;

use crate::components::sidebar::{SidebarContent, SidebarItem, SidebarSection};

// ============================================================================
// Page Path & Selection
// ============================================================================

pub const PATH: &str = "/downloads";

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum Selection {
    #[default]
    All,
    InProgress,
    Completed,
    Failed,
}

impl Selection {
    pub const fn id(&self) -> &'static str {
        match self {
            Self::All => "all",
            Self::InProgress => "in_progress",
            Self::Completed => "completed",
            Self::Failed => "failed",
        }
    }

    pub fn from_id(id: &str) -> Self {
        match id {
            "all" => Self::All,
            "in_progress" => Self::InProgress,
            "completed" => Self::Completed,
            "failed" => Self::Failed,
            _ => Self::default(),
        }
    }
}

// ============================================================================
// Sidebar Content
// ============================================================================

pub const DEFAULT_ID: &str = Selection::All.id();

const STATUS_ITEMS: &[SidebarItem] = &[
    SidebarItem::new("all", "全部"),
    SidebarItem::new("in_progress", "进行中"),
    SidebarItem::new("completed", "已完成"),
    SidebarItem::new("failed", "失败"),
];

pub static SIDEBAR: SidebarContent =
    SidebarContent::new(&[SidebarSection::new(STATUS_ITEMS).with_title("状态")]);

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
            Selection::All => "所有下载任务",
            Selection::InProgress => "正在下载的任务",
            Selection::Completed => "已完成的下载",
            Selection::Failed => "下载失败的任务",
        };

        div()
            .flex()
            .flex_col()
            .gap(px(10.0))
            .child(
                div()
                    .text_size(px(20.0))
                    .child(SharedString::from("下载中心")),
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
