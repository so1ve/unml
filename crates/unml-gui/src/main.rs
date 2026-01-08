mod app;
mod components;
mod pages;

use gpui::*;
use gpui_component_assets::Assets;

fn main() {
    Application::new().with_assets(Assets).run(|cx: &mut App| {
        gpui_component::init(cx);
        gpui_router::init(cx);

        let bounds = Bounds::centered(None, size(px(960.), px(600.)), cx);

        cx.open_window(
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

        cx.activate(true);
    });
}
