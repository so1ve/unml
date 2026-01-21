mod browse;
mod filter;
mod installed;

use browse::BrowseModsPage;
use filter::{FabricModsPage, ForgeModsPage, QuiltModsPage};
use installed::InstalledModsPage;
use unml_macros::PageRoute;

#[derive(PageRoute)]
#[route(id = "mods", label = "nav.mods", icon = Star)]
#[sidebar(
    variant = Filter,
    section "mods.view" {
        Installed => "mods.installed",
        Browse => "mods.browse",
    },
    section "mods.filter" {
        Fabric => "mods.fabric",
        Forge => "mods.forge",
        Quilt => "mods.quilt",
    }
)]
#[children(
    InstalledModsPage,
    BrowseModsPage,
    FabricModsPage,
    ForgeModsPage,
    QuiltModsPage
)]
pub struct ModsPage;
