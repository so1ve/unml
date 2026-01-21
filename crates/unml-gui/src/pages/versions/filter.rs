use gpui::*;
use gpui_component::ActiveTheme;
use gpui_markup::ui;
use rust_i18n::t;
use unml_macros::SubRoute;

use crate::routing::PageView;

#[derive(SubRoute)]
#[subroute(id = "Release")]
#[layout(title = "versions.release")]
pub struct ReleaseVersionsPage;

impl PageView for ReleaseVersionsPage {
    fn view(_window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        ui! {
            div @[text_color: theme.muted_foreground] {
                t!("versions.release_placeholder").to_string()
            }
        }
    }
}

#[derive(SubRoute)]
#[subroute(id = "Snapshot")]
#[layout(title = "versions.snapshot")]
pub struct SnapshotVersionsPage;

impl PageView for SnapshotVersionsPage {
    fn view(_window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        ui! {
            div @[text_color: theme.muted_foreground] {
                t!("versions.snapshot_placeholder").to_string()
            }
        }
    }
}

#[derive(SubRoute)]
#[subroute(id = "Old")]
#[layout(title = "versions.old")]
pub struct OldVersionsPage;

impl PageView for OldVersionsPage {
    fn view(_window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        ui! {
            div @[text_color: theme.muted_foreground] {
                t!("versions.old_placeholder").to_string()
            }
        }
    }
}

#[derive(SubRoute)]
#[subroute(id = "Installed")]
#[layout(title = "versions.installed_only")]
pub struct InstalledVersionsPage;

impl PageView for InstalledVersionsPage {
    fn view(_window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        ui! {
            div @[text_color: theme.muted_foreground] {
                t!("versions.installed_placeholder").to_string()
            }
        }
    }
}
