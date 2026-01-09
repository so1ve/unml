use gpui::prelude::*;
use gpui::*;
use gpui_component::ActiveTheme;
use gpui_router::{IntoLayout, Outlet};
use rust_i18n::t;

// ============================================================================
// Home Layout - Layout with account panel sidebar
// ============================================================================

#[derive(IntoElement, IntoLayout)]
pub struct HomeLayout {
    outlet: Outlet,
}

impl HomeLayout {
    pub fn new() -> Self {
        Self {
            outlet: Outlet::new(),
        }
    }
}

impl RenderOnce for HomeLayout {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_1()
            .overflow_hidden()
            .child(AccountSidebar)
            .child(
                div()
                    .id("content")
                    .flex()
                    .flex_col()
                    .flex_1()
                    .bg(cx.theme().background)
                    .text_color(cx.theme().foreground)
                    .p(px(16.0))
                    .child(self.outlet),
            )
    }
}

// ============================================================================
// Account Sidebar
// ============================================================================

#[derive(IntoElement)]
struct AccountSidebar;

impl RenderOnce for AccountSidebar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        div()
            .id("account-panel")
            .w(px(240.0))
            .h_full()
            .bg(theme.sidebar)
            .border_r_1()
            .border_color(theme.border)
            .flex()
            .flex_col()
            .items_center()
            .p_4()
            .gap_4()
            // Avatar placeholder (96x96)
            .child(
                div()
                    .w(px(96.0))
                    .h(px(96.0))
                    .rounded(px(8.0))
                    .bg(theme.primary)
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(theme.primary_foreground)
                    .text_xl()
                    .child(SharedString::from("S")),
            )
            // Username
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(theme.foreground)
                    .child(SharedString::from("Steve")),
            )
            // Account type label
            .child(
                div()
                    .text_sm()
                    .text_color(theme.muted_foreground)
                    .child(t!("account.microsoft").to_string()),
            )
            // Account selector
            .child(
                div()
                    .w_full()
                    .px_2()
                    .child(
                        div()
                            .w_full()
                            .h(px(36.0))
                            .px_3()
                            .rounded(px(6.0))
                            .bg(theme.secondary)
                            .border_1()
                            .border_color(theme.border)
                            .hover(|s| s.bg(theme.secondary_hover))
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .justify_between()
                            .text_color(theme.foreground)
                            .child(t!("account.steve_microsoft").to_string())
                            .child(SharedString::from("▼")),
                    ),
            )
            // Divider
            .child(div().w_full().h(px(1.0)).bg(theme.border).my_2())
            // Add account button
            .child(
                div()
                    .w_full()
                    .px_2()
                    .child(
                        div()
                            .w_full()
                            .h(px(36.0))
                            .rounded(px(6.0))
                            .bg(theme.secondary)
                            .hover(|s| s.bg(theme.secondary_active))
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .justify_center()
                            .gap_2()
                            .text_color(theme.muted_foreground)
                            .child(t!("account.add").to_string()),
                    ),
            )
    }
}
