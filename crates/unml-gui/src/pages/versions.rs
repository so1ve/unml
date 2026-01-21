mod filter;

use filter::{InstalledVersionsPage, OldVersionsPage, ReleaseVersionsPage, SnapshotVersionsPage};
use unml_macros::PageRoute;

#[derive(PageRoute)]
#[route(id = "versions", label = "nav.versions", icon = Folder)]
#[sidebar(
    variant = Filter,
    section "versions.filter" {
        Release => "versions.release",
        Snapshot => "versions.snapshot",
        Old => "versions.old",
    },
    section {
        Installed => "versions.installed_only",
    }
)]
#[children(
    ReleaseVersionsPage,
    SnapshotVersionsPage,
    OldVersionsPage,
    InstalledVersionsPage
)]
pub struct VersionsPage;
