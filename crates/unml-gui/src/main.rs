#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod components;
mod pages;
mod routes;
mod routing;
mod theme;
mod tokio;

use gpui::*;
use gpui_component_assets::Assets;

rust_i18n::i18n!("locales", fallback = "en");

fn init(cx: &mut App) {
    rust_i18n::set_locale("zh-CN");

    tokio::init(cx);

    gpui_component::init(cx);
    gpui_router::init(cx);

    theme::apply_unml_dark_theme(cx);
}

fn main() {
    Application::new().with_assets(Assets).run(|cx: &mut App| {
        init(cx);

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
                |window, cx| {
                    let view = cx.new(|_| app::LauncherView::new());
                    cx.new(|cx| gpui_component::Root::new(view, window, cx))
                },
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
