use gpui::*;

pub fn page() -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap(px(10.0))
        .child(div().text_size(px(20.0)).child("404"))
        .child(div().text_color(rgb(0xa0a0a0)).child("你怎么到这来的?"))
}
