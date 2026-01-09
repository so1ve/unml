use gpui::*;
use gpui_router::use_params;
use rust_i18n::t;

unml_macros::define_sidebar! {
    variant: Filter,

    section "downloads.status" {
        All => "downloads.all",
        InProgress => "downloads.in_progress",
        Completed => "downloads.completed",
        Failed => "downloads.failed",
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
            Selection::All => t!("downloads.all_tasks"),
            Selection::InProgress => t!("downloads.in_progress_tasks"),
            Selection::Completed => t!("downloads.completed_tasks"),
            Selection::Failed => t!("downloads.failed_tasks"),
        };

        div()
            .flex()
            .flex_col()
            .gap(px(10.0))
            .child(
                div()
                    .text_size(px(20.0))
                    .child(t!("downloads.title").to_string()),
            )
            .child(
                div()
                    .text_color(rgb(0xa0a0a0))
                    .child(content.to_string()),
            )
    }
}

pub fn page() -> Page {
    Page
}
