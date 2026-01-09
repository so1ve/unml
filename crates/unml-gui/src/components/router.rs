use gpui::*;
use gpui_router::{Route, Routes};

use crate::components::layout::{HomeLayout, PageLayout};
use crate::components::sidebar::SidebarVariant;
use crate::pages;

pub fn router() -> impl IntoElement {
    Routes::new()
        .basename("/")
        // Home page with account sidebar layout
        .child(
            Route::new()
                .index()
                .layout(HomeLayout::new())
                .child(Route::new().index().element(|_, _| pages::home::page())),
        )
        // Versions with sidebar layout
        .child(
            Route::new()
                .path("versions")
                .layout(PageLayout::new(
                    pages::versions::PATH,
                    &pages::versions::SIDEBAR,
                    SidebarVariant::Filter,
                    pages::versions::DEFAULT_ID,
                ))
                .child(Route::new().index().element(|_, _| pages::versions::page()))
                .child(
                    Route::new()
                        .path("{selection}")
                        .element(|_, _| pages::versions::page()),
                ),
        )
        // Mods with sidebar layout
        .child(
            Route::new()
                .path("mods")
                .layout(PageLayout::new(
                    pages::mods::PATH,
                    &pages::mods::SIDEBAR,
                    SidebarVariant::Filter,
                    pages::mods::DEFAULT_ID,
                ))
                .child(Route::new().index().element(|_, _| pages::mods::page()))
                .child(
                    Route::new()
                        .path("{selection}")
                        .element(|_, _| pages::mods::page()),
                ),
        )
        // Downloads with sidebar layout
        .child(
            Route::new()
                .path("downloads")
                .layout(PageLayout::new(
                    pages::downloads::PATH,
                    &pages::downloads::SIDEBAR,
                    SidebarVariant::Filter,
                    pages::downloads::DEFAULT_ID,
                ))
                .child(Route::new().index().element(|_, _| pages::downloads::page()))
                .child(
                    Route::new()
                        .path("{selection}")
                        .element(|_, _| pages::downloads::page()),
                ),
        )
        // Settings with sidebar layout
        .child(
            Route::new()
                .path("settings")
                .layout(PageLayout::new(
                    pages::settings::PATH,
                    &pages::settings::SIDEBAR,
                    SidebarVariant::Navigation,
                    pages::settings::DEFAULT_ID,
                ))
                .child(Route::new().index().element(|_, _| pages::settings::page()))
                .child(
                    Route::new()
                        .path("{selection}")
                        .element(|_, _| pages::settings::page()),
                ),
        )
        // 404
        .child(
            Route::new()
                .path("{*not_match}")
                .element(|_, _| pages::not_found::page()),
        )
}
