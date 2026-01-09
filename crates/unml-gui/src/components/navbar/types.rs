use gpui_component::IconName;

use crate::routes::paths;

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
    pub default_id: Option<&'static str>,
    pub icon: IconName,
}

impl TabItem {
    pub const fn new(
        id: &'static str,
        label: &'static str,
        active_prefix: &'static str,
        default_id: Option<&'static str>,
        icon: IconName,
    ) -> Self {
        Self {
            id,
            label,
            active_prefix,
            default_id,
            icon,
        }
    }

    pub(crate) fn is_active(&self, pathname: &str) -> bool {
        if self.active_prefix == paths::HOME {
            return pathname == paths::HOME;
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
