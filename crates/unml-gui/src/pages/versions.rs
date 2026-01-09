use gpui::*;
use gpui_component::ActiveTheme;
use gpui_router::use_params;
use rust_i18n::t;

unml_macros::define_sidebar! {
    variant: Filter,

    section "versions.filter" {
        Release => "versions.release",
        Snapshot => "versions.snapshot",
        Old => "versions.old",
    }
    section {
        Installed => "versions.installed_only",
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
            Selection::Release => t!("versions.release_list"),
            Selection::Snapshot => t!("versions.snapshot_list"),
            Selection::Old => t!("versions.old_list"),
            Selection::Installed => t!("versions.installed_list"),
        };

        div()
            .flex()
            .flex_col()
            .gap(px(10.0))
            .child(
                div()
                    .text_size(px(20.0))
                    .child(t!("versions.title").to_string()),
            )
            .child(
                div()
                    .text_color(cx.theme().muted_foreground)
                    .child(content.to_string()),
            )
    }
}

pub fn page() -> Page {
    Page
}
