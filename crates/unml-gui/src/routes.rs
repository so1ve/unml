//! Application route definitions.
//!
//! This module defines all routes and navigation tabs using the
//! `define_app_routes!` macro. The macro generates:
//! - `paths` module with PATH constants
//! - `router()` function
//! - `NAV_TABS` constant

unml_macros::define_app_routes! {
    home {
        label: "nav.home",
        icon: LayoutDashboard,
    }

    versions {
        label: "nav.versions",
        icon: Folder,
    }

    mods {
        label: "nav.mods",
        icon: Star,
    }

    downloads {
        label: "nav.downloads",
        icon: ArrowDown,
    }

    settings {
        label: "nav.settings",
        icon: Settings,

        children {
            java,
        }
    }
}
