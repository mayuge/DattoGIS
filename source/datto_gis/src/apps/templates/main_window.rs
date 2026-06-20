use gpui::*;

use crate::components::atoms::button::*;


pub struct MainTemplate;

impl MainTemplate {
    pub fn render() -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(16.0))
            .p(px(16.0))
            .child(
                Button::new(
                    String::from("保存"),
                    ButtonVariant::Primary,
                    ButtonShape::Rounded,
                    ButtonSize::Medium,
                    None,
                    None,
                )
                .render(),
            )
            .child(
                Button::new(
                    String::from("削除"),
                    ButtonVariant::Danger,
                    ButtonShape::Rounded,
                    ButtonSize::Medium,
                    None,
                    None,
                )
                .render(),
            )
    }
}