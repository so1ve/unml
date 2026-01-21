//! Routing system for the application.

mod builder;
mod traits;

pub use builder::build_route;
pub use traits::{
    ChildRoutes, PageRoute, PageView, PlainRouteDef, Routable, SidebarRouteDef, SubRoute,
};

#[macro_export]
macro_rules! define_routes {
    ($($page:ty),* $(,)?) => {
        pub fn router() -> impl gpui::IntoElement {
            gpui_router::Routes::new()
                .basename("/")
                $(.child($crate::routing::build_route::<$page>()))*
        }

        pub static NAV_TABS: std::sync::LazyLock<Vec<$crate::components::navbar::TabItem>> =
            std::sync::LazyLock::new(|| {
                vec![$($crate::components::navbar::TabItem::from_route(&<$page as $crate::routing::Routable>::route())),*]
            });
    };
}
