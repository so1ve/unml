use gpui::prelude::*;
use gpui::*;
use gpui_component::*;

pub fn titlebar() -> TitleBar {
    TitleBar::new()
}

#[derive(IntoElement)]
pub struct TitleBar;

impl TitleBar {
    pub fn new() -> Self {
        Self
    }
}

impl RenderOnce for TitleBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .id("title-bar")
            .h(px(40.0))
            .bg(cx.theme().title_bar)
            .text_color(cx.theme().foreground)
            .flex()
            .items_center()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .pl_3()
                    .child(
                        div()
                            .w(px(18.0))
                            .h(px(18.0))
                            .rounded(px(4.0))
                            .bg(cx.theme().secondary)
                            .child(
                                div()
                                    .w(px(18.0))
                                    .h(px(18.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(SharedString::from("U")),
                            ),
                    )
                    .child(SharedString::from("UNML")),
            )
            .child(div().flex_1().h(px(40.0)))
            .child(
                div()
                    .flex()
                    .items_center()
                    .pr_2()
                    .child(WindowControls::new()),
            )
            .window_control_area(WindowControlArea::Drag)
    }
}

#[derive(IntoElement)]
struct WindowControls;

impl WindowControls {
    pub fn new() -> Self {
        Self
    }
}

impl RenderOnce for WindowControls {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        if cfg!(target_os = "macos") {
            return div().id("window-controls");
        }

        h_flex()
            .id("window-controls")
            .items_center()
            .gap(px(6.0))
            .flex_shrink_0()
            .h_full()
            .child(ControlIcon::minimize())
            .child(ControlIcon::close())
    }
}

#[derive(IntoElement, Clone)]
enum ControlIcon {
    Minimize,
    Close,
}

impl ControlIcon {
    fn minimize() -> Self {
        Self::Minimize
    }

    fn close() -> Self {
        Self::Close
    }

    fn id(&self) -> &'static str {
        match self {
            Self::Minimize => "minimize",
            Self::Close => "close",
        }
    }

    fn icon(&self) -> IconName {
        match self {
            Self::Minimize => IconName::WindowMinimize,
            Self::Close => IconName::WindowClose,
        }
    }

    fn window_control_area(&self) -> WindowControlArea {
        match self {
            Self::Minimize => WindowControlArea::Min,
            Self::Close => WindowControlArea::Close,
        }
    }

    fn is_close(&self) -> bool {
        matches!(self, Self::Close)
    }
}

impl RenderOnce for ControlIcon {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_linux = cfg!(target_os = "linux");
        let is_windows = cfg!(target_os = "windows");

        let (base_bg, hover_bg, active_bg) = if self.is_close() {
            (0x3a1f1f, 0x532a2a, 0x3f1f1f)
        } else {
            (0x2d2d2d, 0x4c4c4c, 0x2f2f2f)
        };

        let hover_fg = cx.theme().secondary_foreground;
        let active_fg = cx.theme().secondary_foreground;

        let icon = self.clone();

        div()
            .id(self.id())
            .flex()
            .w(px(36.0))
            .h(px(28.0))
            .rounded(px(6.0))
            .bg(rgb(base_bg))
            .flex_shrink_0()
            .justify_center()
            .content_center()
            .items_center()
            .cursor_pointer()
            .text_color(cx.theme().foreground)
            .hover(|style| style.bg(rgb(hover_bg)).text_color(hover_fg))
            .active(|style| style.bg(rgb(active_bg)).text_color(active_fg))
            .when(is_windows, |this| {
                this.window_control_area(self.window_control_area())
            })
            .when(is_linux, |this| {
                this.on_mouse_down(MouseButton::Left, move |_, window, cx| {
                    window.prevent_default();
                    cx.stop_propagation();
                })
                .on_click(move |_, window, cx| {
                    cx.stop_propagation();
                    match icon {
                        Self::Minimize => window.minimize_window(),
                        Self::Close => window.remove_window(),
                    }
                })
            })
            .child(Icon::new(self.icon()).small())
    }
}
