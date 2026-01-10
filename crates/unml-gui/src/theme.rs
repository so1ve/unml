use std::rc::Rc;

use gpui::App;
use gpui_component::Theme;
use gpui_component::theme::{ThemeConfig, ThemeMode};

pub fn apply_unml_dark_theme(cx: &mut App) {
    let mut config = ThemeConfig {
        is_default: true,
        name: "UNML Dark".into(),
        mode: ThemeMode::Dark,
        ..Default::default()
    };

    // Base colors
    config.colors.background = Some("#1a1a1a".into());
    config.colors.foreground = Some("#e8e8e8".into());
    config.colors.border = Some("#2d2d2d".into());

    // Sidebar background
    config.colors.sidebar = Some("#252525".into());
    config.colors.sidebar_foreground = Some("#e8e8e8".into());
    config.colors.sidebar_border = Some("#2d2d2d".into());

    // Title bar
    config.colors.title_bar = Some("#1e1e1e".into());
    config.colors.title_bar_border = Some("#2d2d2d".into());

    // Popover
    config.colors.popover = Some("#1e1e1e".into());
    config.colors.popover_foreground = Some("#e8e8e8".into());

    // Secondary (used for cards, inputs, etc.)
    config.colors.secondary = Some("#2d2d2d".into());
    config.colors.secondary_hover = Some("#353535".into());
    config.colors.secondary_active = Some("#3d3d3d".into());
    config.colors.secondary_foreground = Some("#e8e8e8".into());

    // Muted colors
    config.colors.muted = Some("#2d2d2d".into());
    config.colors.muted_foreground = Some("#a0a0a0".into());

    // Primary/accent color (blue)
    config.colors.primary = Some("#3b82f6".into());
    config.colors.primary_hover = Some("#2563eb".into());
    config.colors.primary_active = Some("#1d4ed8".into());
    config.colors.primary_foreground = Some("#ffffff".into());

    // Accent
    config.colors.accent = Some("#3b82f6".into());
    config.colors.accent_foreground = Some("#ffffff".into());

    // Tab bar
    config.colors.tab_bar = Some("#252525".into());
    config.colors.tab = Some("#252525".into());
    config.colors.tab_active = Some("#3c3c3c".into());
    config.colors.tab_active_foreground = Some("#e8e8e8".into());
    config.colors.tab_foreground = Some("#a0a0a0".into());

    // List
    config.colors.list = Some("#252525".into());
    config.colors.list_active = Some("#3c3c3c".into());
    config.colors.list_active_border = Some("#3b82f6".into());
    config.colors.list_hover = Some("#353535".into());

    // Ring (focus indicator)
    config.colors.ring = Some("#3b82f6".into());

    // Scrollbar
    config.colors.scrollbar = Some("#252525".into());
    config.colors.scrollbar_thumb = Some("#3d3d3d".into());
    config.colors.scrollbar_thumb_hover = Some("#4d4d4d".into());

    Theme::global_mut(cx).apply_config(&Rc::new(config));
    Theme::change(ThemeMode::Dark, None, cx);
}
