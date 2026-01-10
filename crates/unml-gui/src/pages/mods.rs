use gpui::*;
use gpui_component::ActiveTheme;
use gpui_markup::ui;
use gpui_router::use_params;
use rust_i18n::t;

unml_macros::define_sidebar! {
    variant: Filter,

    section "mods.view" {
        Installed => "mods.installed",
        Browse => "mods.browse",
    }
    section "mods.filter" {
        Fabric => "mods.fabric",
        Forge => "mods.forge",
        Quilt => "mods.quilt",
    }
}

// ============================================================================
// Page Content
// ============================================================================

#[derive(IntoElement)]
pub struct Page;

impl RenderOnce for Page {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let params = use_params(cx);
        let selection = params
            .get("selection")
            .map(|s| Selection::from_id(s))
            .unwrap_or_default();

        let content = match selection {
            Selection::Installed => t!("mods.installed_list"),
            Selection::Browse => t!("mods.browse_online"),
            Selection::Fabric => t!("mods.fabric"),
            Selection::Forge => t!("mods.forge"),
            Selection::Quilt => t!("mods.quilt"),
        };

        let theme = cx.theme();

        ui! {
            <div flex flex_col gap={px(10.0)}>
                <div text_size={px(20.0)}>
                    {t!("mods.title").to_string()}
                </div>
                <div text_color={theme.muted_foreground}>
                    {content.to_string()}
                </div>
            </div>
        }
    }
}

pub fn page() -> Page {
    Page
}
