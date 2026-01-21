use gpui_component::IconName;

use crate::routing::PageRoute;

#[derive(Clone)]
pub enum TabKind {
    Plain,
    Sidebar { default_id: &'static str },
}

#[derive(Clone)]
pub struct TabItem {
    pub id: &'static str,
    pub label: &'static str,
    pub icon: Option<IconName>,
    pub kind: TabKind,
}

impl TabItem {
    pub fn from_route(route: &PageRoute) -> Self {
        match route {
            PageRoute::Plain(def) => Self {
                id: def.id,
                label: def.label,
                icon: def.icon.clone(),
                kind: TabKind::Plain,
            },
            PageRoute::Sidebar(def) => Self {
                id: def.id,
                label: def.label,
                icon: def.icon.clone(),
                kind: TabKind::Sidebar {
                    default_id: def.default_id,
                },
            },
        }
    }

    pub fn to_path(&self) -> String {
        match &self.kind {
            TabKind::Plain => format!("/{}", self.id),
            TabKind::Sidebar { default_id } => format!("/{}/{}", self.id, default_id),
        }
    }

    pub(crate) fn is_active(&self, pathname: &str) -> bool {
        let pathname = pathname.strip_prefix('/').unwrap_or(pathname);

        if !pathname.starts_with(self.id) {
            return false;
        }

        let id_len = self.id.len();

        pathname.len() == id_len || pathname.as_bytes().get(id_len) == Some(&b'/')
    }
}
