use gpui::*;
use gpui_component::ActiveTheme;
use gpui_markup::ui;

use crate::components::navbar::NavBar;
use crate::components::titlebar::TitleBar;
use crate::routes;

#[derive(Clone)]
pub struct LauncherView;

impl LauncherView {
    pub fn new() -> Self {
        Self
    }
}

impl Render for LauncherView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        ui! {
            <div size_full bg={cx.theme().background} flex flex_col>
                <TitleBar />
                <NavBar />
                <div flex flex_1 overflow_hidden bg={cx.theme().background} text_color={cx.theme().foreground}>
                    {routes::router()}
                </div>
            </div>
        }
    }
}
