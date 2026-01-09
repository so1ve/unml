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
        label: "首页",
        icon: LayoutDashboard,
    }

    versions {
        path: "/versions",
        label: "版本",
        icon: Folder,
        sidebar_variant: Filter,
    }

    mods {
        path: "/mods",
        label: "Mod",
        icon: Star,
        sidebar_variant: Filter,
    }

    downloads {
        path: "/downloads",
        label: "下载",
        icon: ArrowDown,
        sidebar_variant: Filter,
    }

    settings {
        path: "/settings",
        label: "设置",
        icon: Settings,
        sidebar_variant: Navigation,
    }
}
