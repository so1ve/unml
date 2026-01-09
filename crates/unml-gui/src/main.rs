mod app;
mod components;
mod pages;
mod theme;

use gpui::*;
use gpui_component_assets::Assets;

fn main() {
    Application::new().with_assets(Assets).run(|cx: &mut App| {
        gpui_component::init(cx);
        gpui_router::init(cx);

        theme::apply_unml_dark_theme(cx);

        let bounds = Bounds::centered(None, size(px(960.), px(600.)), cx);

        let window = cx
            .open_window(
                WindowOptions {
                    window_decorations: Some(WindowDecorations::Client),
                    titlebar: Some(gpui_component::TitleBar::title_bar_options()),
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    is_resizable: false,
                    ..Default::default()
                },
                |_, cx| cx.new(|_| app::LauncherView::new()),
            )
            .unwrap();

        window
            .update(cx, |_, window, _cx| {
                window.set_window_title("UNML");
            })
            .expect("failed to update window");

        cx.activate(true);
    });
}
