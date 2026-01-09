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

    config.colors.background = Some("#1a1a1a".into());
    config.colors.foreground = Some("#e8e8e8".into());
    config.colors.border = Some("#2d2d2d".into());

    config.colors.title_bar = Some("#1e1e1e".into());
    config.colors.title_bar_border = Some("#2d2d2d".into());

    config.colors.popover = Some("#1e1e1e".into());
    config.colors.popover_foreground = Some("#e8e8e8".into());

    config.colors.secondary = Some("#2d2d2d".into());
    config.colors.secondary_hover = Some("#4c4c4c".into());
    config.colors.secondary_active = Some("#2f2f2f".into());
    config.colors.secondary_foreground = Some("#e8e8e8".into());

    // Keep muted text readable on dark background.
    config.colors.muted_foreground = Some("#e8e8e8".into());

    Theme::global_mut(cx).apply_config(&Rc::new(config));
    Theme::change(ThemeMode::Dark, None, cx);
}
