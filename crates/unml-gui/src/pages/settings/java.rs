use std::path::PathBuf;
use std::sync::Arc;

use gpui::*;
use gpui_component::ActiveTheme;
use gpui_component::button::Button;
use gpui_component::select::{Select, SelectItem, SelectState};
use gpui_markup::ui;
use rust_i18n::t;
use unml_java::{JavaInstallation, JavaManager};
use unml_macros::SubRoute;

use crate::tokio::Tokio;

#[derive(Debug, Clone)]
struct JavaOption {
    display_name: SharedString,
    path: PathBuf,
    version: String,
    vendor: Option<String>,
}

impl JavaOption {
    fn from_installation(installation: &JavaInstallation) -> Self {
        let vendor_str = if let Some(vendor) = &installation.vendor {
            format!(" ({vendor}) ")
        } else {
            " ".to_owned()
        };

        let display = format!(
            "Java {}{}— {}",
            installation.major_version,
            vendor_str,
            installation.executable.display()
        );

        Self {
            display_name: display.into(),
            path: installation.executable.clone(),
            version: installation.version.clone(),
            vendor: installation.vendor.clone(),
        }
    }
}

impl SelectItem for JavaOption {
    type Value = PathBuf;

    fn title(&self) -> SharedString {
        self.display_name.clone()
    }

    fn value(&self) -> &Self::Value {
        &self.path
    }

    fn matches(&self, query: &str) -> bool {
        let query_lower = query.to_lowercase();
        self.display_name.to_lowercase().contains(&query_lower)
            || self.version.to_lowercase().contains(&query_lower)
            || self
                .vendor
                .as_ref()
                .map(|v| v.to_lowercase().contains(&query_lower))
                .unwrap_or(false)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum JavaLoadState {
    Loading,
    Loaded,
    Error(String),
}

#[derive(SubRoute)]
#[subroute(path = "/settings/java")]
pub struct JavaSettingsPage;

impl JavaSettingsPage {
    pub fn view(window: &mut Window, cx: &mut App) -> impl IntoElement {
        cx.new(|cx| JavaSettingsView::new(window, cx))
    }
}

pub struct JavaSettingsView {
    java_manager: Arc<JavaManager>,
    select_state: Entity<SelectState<Vec<JavaOption>>>,
    load_state: JavaLoadState,
    #[allow(dead_code)]
    selected_java: Option<PathBuf>,
}

impl JavaSettingsView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let java_manager = Arc::new(JavaManager::new());

        let select_state = cx.new(|cx| {
            SelectState::new(Vec::<JavaOption>::new(), None, window, cx).searchable(true)
        });

        let mut view = Self {
            java_manager,
            select_state,
            load_state: JavaLoadState::Loading,
            selected_java: None,
        };

        view.load_java_installations(window, cx);

        view
    }

    fn load_java_installations(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.load_state = JavaLoadState::Loading;
        cx.notify();

        let manager = self.java_manager.clone();

        let task = Tokio::spawn(cx, async move { manager.get_installations().await });

        cx.spawn_in(window, async move |this, cx| {
            let result = task.await;

            cx.update(|window, cx| {
                this.update(cx, |view, cx| {
                    match result {
                        Ok(Ok(installations)) => {
                            let options: Vec<JavaOption> = installations
                                .iter()
                                .map(JavaOption::from_installation)
                                .collect();

                            view.load_state = JavaLoadState::Loaded;

                            view.select_state.update(cx, |state, cx| {
                                state.set_items(options, window, cx);
                            });
                        }
                        Ok(Err(e)) => {
                            view.load_state = JavaLoadState::Error(e.to_string());
                        }
                        Err(e) => {
                            view.load_state = JavaLoadState::Error(e.to_string());
                        }
                    }

                    cx.notify();
                })
            })
            .ok();
        })
        .detach();
    }

    fn refresh_java_installations(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let manager = self.java_manager.clone();

        let task = Tokio::spawn(cx, async move {
            manager.clear_cache().await;
        });

        cx.spawn_in(window, async move |_this, _cx| {
            task.await.ok();
        })
        .detach();

        self.load_java_installations(window, cx);
    }
}

impl Render for JavaSettingsView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        let content: Div = match &self.load_state {
            JavaLoadState::Loading => ui! {
                div @[flex, items_center, gap_2, text_color: theme.muted_foreground] {
                    t!("settings.java_loading").to_string()
                }
            },
            JavaLoadState::Error(err) => {
                let error_msg = format!("Error: {}", err);
                let refresh_btn = Button::new("refresh-btn")
                    .label(t!("settings.java_refresh").to_string())
                    .on_click(cx.listener(|view, _event, window, cx| {
                        view.refresh_java_installations(window, cx);
                    }));

                ui! {
                    div @[flex, flex_col, gap_2] {
                        div @[text_color: theme.danger] {
                            error_msg
                        },
                        refresh_btn
                    }
                }
            }
            JavaLoadState::Loaded => {
                ui! {
                    div @[flex, flex_col, gap_4] {
                        div @[flex, flex_col, gap_2] {
                            div @[text_sm, text_color: theme.foreground] {
                                t!("settings.java_select_label").to_string()
                            },
                            div @[w_full, min_w_0, overflow_hidden] {
                                Select::new(&self.select_state) @[
                                    placeholder: t!("settings.java_select_placeholder").to_string(),
                                    cleanable: true,
                                ] {}
                            }
                        },
                        Button::new("refresh-btn") @[
                            label: t!("settings.java_refresh").to_string(),
                            on_click: cx.listener(|view, _event, window, cx| {
                                view.refresh_java_installations(window, cx);
                            })
                        ] {}
                    }
                }
            }
        };

        ui! {
            div @[flex, flex_col, gap: px(16.)] {
                div @[text_xl, font_weight: FontWeight::SEMIBOLD] {
                    t!("settings.java_title").to_string()
                },
                div @[text_color: theme.muted_foreground] {
                    t!("settings.java_desc").to_string()
                },
                content
            }
        }
    }
}
