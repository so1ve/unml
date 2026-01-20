use gpui_component::IconName;

use crate::routing::PageRoute;

/// A tab item definition for the navigation bar
#[derive(Clone)]
pub struct TabItem {
    pub id: &'static str,
    pub label: &'static str,
    /// Route ID used to determine active state (e.g., "versions", "" for home)
    pub active_id: &'static str,
    /// Default selection id (e.g. "release"), used to build link target
    /// `/{active_id}/{default_id}`. If None, link target is `/{active_id}`.
    pub default_id: &'static str,
    /// Optional icon - may be None for child routes or routes without icons
    pub icon: Option<IconName>,
}

impl TabItem {
    pub const fn from_page<P: PageRoute>() -> Self {
        Self {
            id: P::ID,
            label: P::LABEL,
            active_id: P::ID,
            default_id: P::DEFAULT_ID,
            icon: P::ICON,
        }
    }

    pub(crate) fn is_active(&self, pathname: &str) -> bool {
        // Home page: active_id is ""
        if self.active_id.is_empty() {
            return pathname == "/";
        }

        let pathname = pathname.strip_prefix('/').unwrap_or(pathname);

        if !pathname.starts_with(self.active_id) {
            return false;
        }

        // Ensure path boundary: "versions" matches "versions" and "versions/...",
        // but not "versions2".
        let id_len = self.active_id.len();

        pathname.len() == id_len || pathname.as_bytes().get(id_len) == Some(&b'/')
    }
}
