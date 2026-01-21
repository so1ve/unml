use gpui::*;
use gpui_component::ActiveTheme;
use gpui_markup::ui;
use rust_i18n::t;
use unml_macros::SubRoute;

use crate::routing::PageView;

#[derive(SubRoute)]
#[subroute(id = "Completed")]
#[layout(title = "downloads.completed")]
pub struct CompletedPage;

impl PageView for CompletedPage {
    fn view(_window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        ui! {
            div @[text_color: theme.muted_foreground] {
                t!("downloads.completed_placeholder").to_string()
            }
        }
    }
}
