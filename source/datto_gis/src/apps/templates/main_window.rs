use gpui::*;

use crate::domain::app_config::*;
use crate::components::atoms::button::*;
use crate::components::atoms::header::*;

pub struct MainTemplate;

impl MainTemplate {
    pub fn render() -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .child(Header::new(String::from(APP_NAME)).render())
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