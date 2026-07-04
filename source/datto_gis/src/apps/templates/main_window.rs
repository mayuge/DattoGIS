use gpui::*;

use crate::domain::app_config::*;

use crate::apps::organisms::main_window::map_app::MapApp;

use crate::components::atoms::button::*;
use crate::components::molecules::header::*;

pub struct MainTemplate;

impl MainTemplate {
    pub fn render(window: &mut Window) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .child(Header::new(APP_NAME.to_string()).render(window))
            .child(
                div()
                    .flex_1() // ヘッダー以外の領域をすべて使用
                    .child(MapApp::render(window)),
            )
    }
}
