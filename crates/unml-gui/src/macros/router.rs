//! Router macro for defining application routes.

/// Macro to define application routes and navigation tabs in one place.
///
/// This macro generates:
/// - `pub mod paths` - PATH constants for each route
/// - `pub fn router() -> impl IntoElement` - the application router
/// - `pub const NAV_TABS: &[TabItem]` - navigation tabs for the navbar
///
/// # Example
///
/// ```ignore
/// define_app_routes! {
///     home {
///         path: "/",
///         label: "首页",
///         icon: LayoutDashboard,
///     }
///
///     versions {
///         path: "/versions",
///         label: "版本",
///         icon: Folder,
///         sidebar_variant: Filter,
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_app_routes {
    (
        home {
            path: $home_path:literal,
            label: $home_label:literal,
            icon: $home_icon:ident,
        }

        $(
            $name:ident {
                path: $path:literal,
                label: $label:literal,
                icon: $icon:ident,
                sidebar_variant: $variant:ident,
            }
        )*
    ) => {
        use gpui::IntoElement;
        use gpui_component::IconName;
        use gpui_router::{Route, Routes};

        use $crate::components::layout::{HomeLayout, PageLayout};
        use $crate::components::navbar::TabItem;
        use $crate::components::sidebar::SidebarVariant;
        use $crate::pages;

        /// Path constants for all routes.
        pub mod paths {
            pub const HOME: &str = $home_path;
            $(
                #[allow(non_upper_case_globals)]
                pub const $name: &str = $path;
            )*
        }

        pub fn router() -> impl IntoElement {
            Routes::new()
                .basename("/")
                // Home route (uses HomeLayout)
                .child(
                    Route::new()
                        .index()
                        .layout(HomeLayout::new())
                        .child(Route::new().index().element(|_, _| pages::home::page())),
                )
                // Standard routes (use PageLayout with sidebar)
                $(
                    .child(
                        Route::new()
                            .path(stringify!($name))
                            .layout(PageLayout::new(
                                paths::$name,
                                &pages::$name::SIDEBAR,
                                SidebarVariant::$variant,
                                pages::$name::DEFAULT_ID,
                            ))
                            .child(Route::new().index().element(|_, _| pages::$name::page()))
                            .child(
                                Route::new()
                                    .path("{selection}")
                                    .element(|_, _| pages::$name::page()),
                            ),
                    )
                )*
                // Not found fallback
                .child(
                    Route::new()
                        .path("{*not_match}")
                        .element(|_, _| pages::not_found::page()),
                )
        }

        pub const NAV_TABS: &[TabItem] = &[
            TabItem::new(
                "home",
                $home_label,
                paths::HOME,
                None,
                IconName::$home_icon,
            ),
            $(
                TabItem::new(
                    stringify!($name),
                    $label,
                    paths::$name,
                    Some(pages::$name::DEFAULT_ID),
                    IconName::$icon,
                ),
            )*
        ];
    };
}
