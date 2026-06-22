use gpui::*;

use crate::components::atoms::button::*;
use crate::components::molecules::header::*;
use crate::domain::app_config::*;

pub struct MainTemplate;

impl MainTemplate {
    pub fn render() -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .child(Header::new(APP_NAME.to_string()).render())
            .child(div().flex_grow())
    }
}
