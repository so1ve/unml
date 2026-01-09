use gpui::prelude::*;
use gpui::*;
use gpui_component::ActiveTheme;

#[derive(IntoElement)]
pub struct InstanceCard {
    name: SharedString,
    version: SharedString,
    loader: SharedString,
    icon_color: Rgba,
}

impl InstanceCard {
    pub fn new(
        name: impl Into<SharedString>,
        version: impl Into<SharedString>,
        loader: impl Into<SharedString>,
        icon_color: Rgba,
    ) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            loader: loader.into(),
            icon_color,
        }
    }
}

impl RenderOnce for InstanceCard {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        div()
            .flex()
            .flex_col()
            .w(px(200.0))
            .bg(theme.secondary)
            .hover(|s| s.bg(theme.secondary_hover))
            .border_1()
            .border_color(theme.border)
            .rounded_md()
            .cursor_pointer()
            .child(
                // Icon / Image area
                div()
                    .w_full()
                    .h(px(120.0))
                    .bg(self.icon_color)
                    .rounded_t_md()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_size(px(40.0))
                            .text_color(rgb(0xffffff))
                            .child(self.name.chars().next().unwrap_or('?').to_string()),
                    ),
            )
            .child(
                // Content area
                div()
                    .flex()
                    .flex_col()
                    .p_3()
                    .gap_2()
                    .child(
                        div()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.foreground)
                            .child(self.name.clone()),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .px_1()
                                    .py_0p5()
                                    .bg(cx.theme().background)
                                    .rounded_sm()
                                    .text_size(px(12.0))
                                    .text_color(theme.muted_foreground)
                                    .child(self.version),
                            )
                            .child(
                                div()
                                    .px_1()
                                    .py_0p5()
                                    .bg(cx.theme().background)
                                    .rounded_sm()
                                    .text_size(px(12.0))
                                    .text_color(theme.muted_foreground)
                                    .child(self.loader),
                            ),
                    ),
            )
    }
}
