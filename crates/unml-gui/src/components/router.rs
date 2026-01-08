use gpui::*;
use gpui_router::{Route, Routes};

use crate::pages;

pub fn router() -> impl IntoElement {
    Routes::new()
        .basename("/")
        .child(Route::new().index().element(|_, _| pages::home::page()))
        .child(
            Route::new()
                .path("versions")
                .element(|_, _| pages::versions::page()),
        )
        .child(
            Route::new()
                .path("mods")
                .element(|_, _| pages::mods::page()),
        )
        .child(
            Route::new()
                .path("downloads")
                .element(|_, _| pages::downloads::page()),
        )
        .child(
            Route::new()
                .path("settings")
                .element(|_, _| pages::settings::page()),
        )
        .child(
            Route::new()
                .path("{*not_match}")
                .element(|_, _| pages::not_found::page()),
        )
}
