use gpui::*;
use gpui_router::{Route, Routes};

use crate::pages;

pub fn router() -> impl IntoElement {
    Routes::new()
        .basename("/")
        // Home page
        .child(Route::new().index().element(|_, _| pages::home::page()))
        // Versions with optional selection parameter
        .child(
            Route::new()
                .path("versions")
                .element(|_, _| pages::versions::page()),
        )
        .child(
            Route::new()
                .path("versions/{selection}")
                .element(|_, _| pages::versions::page()),
        )
        // Mods with optional selection parameter
        .child(
            Route::new()
                .path("mods")
                .element(|_, _| pages::mods::page()),
        )
        .child(
            Route::new()
                .path("mods/{selection}")
                .element(|_, _| pages::mods::page()),
        )
        // Downloads with optional selection parameter
        .child(
            Route::new()
                .path("downloads")
                .element(|_, _| pages::downloads::page()),
        )
        .child(
            Route::new()
                .path("downloads/{selection}")
                .element(|_, _| pages::downloads::page()),
        )
        // Settings with optional selection parameter
        .child(
            Route::new()
                .path("settings")
                .element(|_, _| pages::settings::page()),
        )
        .child(
            Route::new()
                .path("settings/{selection}")
                .element(|_, _| pages::settings::page()),
        )
        // 404
        .child(
            Route::new()
                .path("{*not_match}")
                .element(|_, _| pages::not_found::page()),
        )
}
