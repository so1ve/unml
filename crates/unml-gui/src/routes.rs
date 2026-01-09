//! Application route definitions.
//!
//! This module defines all routes and navigation tabs using the
//! `define_app_routes!` macro. The macro generates:
//! - `paths` module with PATH constants
//! - `router()` function
//! - `NAV_TABS` constant

crate::define_app_routes! {
    home {
        path: "/",
        label: "nav.home",
        icon: LayoutDashboard,
    }

    versions {
        path: "/versions",
        label: "nav.versions",
        icon: Folder,
    }

    mods {
        path: "/mods",
        label: "nav.mods",
        icon: Star,
    }

    downloads {
        path: "/downloads",
        label: "nav.downloads",
        icon: ArrowDown,
    }

    settings {
        path: "/settings",
        label: "nav.settings",
        icon: Settings,
    }
}
