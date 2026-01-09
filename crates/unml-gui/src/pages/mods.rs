use gpui::*;
use gpui_router::use_params;

unml_macros::define_sidebar! {
    variant: Filter,

    section "视图" {
        Installed => "已安装",
        Browse => "浏览",
    }
    section "筛选" {
        Fabric => "Fabric",
        Forge => "Forge",
        Quilt => "Quilt",
    }
}

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
