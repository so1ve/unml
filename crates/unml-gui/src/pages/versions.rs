use gpui::*;
use gpui_router::use_params;

unml_macros::define_sidebar! {
    variant: Filter,

    section "筛选" {
        Release => "正式版",
        Snapshot => "快照版",
        Old => "远古版本",
    }
    section {
        Installed => "仅已安装",
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
