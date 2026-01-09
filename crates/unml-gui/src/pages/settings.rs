use gpui::*;
use gpui_router::use_params;

unml_macros::define_sidebar! {
    variant: Navigation,

    section {
        General => "通用",
        Java => "Java",
        Game => "游戏",
        Download => "下载",
        About => "关于",
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

        let (title, content) = match selection {
            Selection::General => ("通用设置", "语言、主题等通用设置"),
            Selection::Java => ("Java 设置", "Java 路径和版本管理"),
            Selection::Game => ("游戏设置", "默认内存、窗口大小等"),
            Selection::Download => ("下载设置", "并发数、镜像源选择"),
            Selection::About => ("关于", "版本信息和更新"),
        };

        div()
            .flex()
            .flex_col()
            .gap(px(10.0))
            .child(div().text_size(px(20.0)).child(SharedString::from(title)))
            .child(
                div()
                    .text_color(rgb(0xa0a0a0))
                    .child(SharedString::from(content)),
            )
    }
}

pub fn page() -> Page {
    Page
}
