use gpui::*;

pub fn page() -> impl IntoElement {
    page_scaffold("版本管理", "Versions (mock)")
}

fn page_scaffold(title: &str, subtitle: &str) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap(px(10.0))
        .child(div().text_size(px(20.0)).child(SharedString::from(title.to_owned())))
        .child(
            div()
                .text_color(rgb(0xa0a0a0))
                .child(SharedString::from(subtitle.to_owned())),
        )
}
