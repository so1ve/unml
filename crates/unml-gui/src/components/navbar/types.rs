use gpui_component::IconName;

use crate::routing::PageRoute;

/// A tab item definition for the navigation bar
#[derive(Clone)]
pub struct TabItem {
    pub id: &'static str,
    pub label: &'static str,
    /// Path prefix used to determine active state (base path like "/versions")
    pub active_prefix: &'static str,
    /// Default selection id (e.g. "release"), used to build link target
    /// `/{active_prefix}/{default_id}`. If None, link target is
    /// `active_prefix`.
    pub default_id: &'static str,
    /// Optional icon - may be None for child routes or routes without icons
    pub icon: Option<IconName>,
}

impl TabItem {
    pub const fn from_page<P: PageRoute>() -> Self {
        Self {
            id: P::PATH,
            label: P::LABEL,
            active_prefix: P::PATH,
            default_id: P::DEFAULT_ID,
            icon: P::ICON,
        }
    }

    pub(crate) fn is_active(&self, pathname: &str) -> bool {
        if self.active_prefix == "/" {
            return pathname == "/";
        }

        if !pathname.starts_with(self.active_prefix) {
            return false;
        }

        // Ensure path boundary: "/versions" matches "/versions" and "/versions/...",
        // but not "/versions2".
        let prefix_len = self.active_prefix.len();

        pathname.len() == prefix_len || pathname.as_bytes().get(prefix_len) == Some(&b'/')
    }
}
