pub mod java;

use gpui::*;
use gpui_markup::ui;
pub use java::init as init_java_settings;
use rust_i18n::t;

unml_macros::define_sidebar! {
    variant: Navigation,

    section {
        java => "settings.java",
    }
}

#[derive(IntoElement)]
pub struct Page;

impl RenderOnce for Page {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        ui! {
            div @[flex, items_center, justify_center, size_full, text_lg] {
                t!("settings.select_category").to_string()
            }
        }
    }
}

pub fn page() -> Page {
    Page
}
