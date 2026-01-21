//! Routing system for the application.

mod builder;
mod traits;

pub use builder::build_route;
pub use traits::{ChildRoutes, PageKind, PageRoute, PageView, SubRoute};

/// Define routes and navigation tabs from a list of page types.
#[macro_export]
macro_rules! define_routes {
    ($($page:ty),* $(,)?) => {
        pub fn router() -> impl gpui::IntoElement {
            gpui_router::Routes::new()
                .basename("/")
                $(.child($crate::routing::build_route::<$page>()))*
        }

        pub const NAV_TABS: &[$crate::components::navbar::TabItem] =
            &[$($crate::components::navbar::TabItem::from_page::<$page>()),*];
    };
}
