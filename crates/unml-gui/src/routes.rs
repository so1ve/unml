//! Application route definitions.
//!
//! This module defines all routes and navigation tabs using the trait-based
//! routing system. Each page implements the `PageRoute` trait via the
//! `#[derive(PageRoute)]` macro.

use crate::pages::{DownloadsPage, HomePage, ModsPage, SettingsPage, VersionsPage};

crate::define_routes![
    HomePage,
    VersionsPage,
    ModsPage,
    DownloadsPage,
    SettingsPage,
];
