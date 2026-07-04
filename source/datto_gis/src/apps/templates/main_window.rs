use gpui::*;

use crate::domain::app_config::*;
use crate::domain::design_token_config::HEADER_HEIGHT;

use crate::apps::organisms::main_window::map_app::MapApp;

use crate::components::atoms::button::*;
use crate::components::molecules::header::*;

pub struct MainTemplate;

impl MainTemplate {
    pub fn render(window: &mut Window) -> impl IntoElement {
        div()
            .relative()
            .size_full()
            .child(div().absolute().inset_0().child(MapApp::render(window)))
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .right_0()
                    .h(px(HEADER_HEIGHT))
                    .child(Header::new(APP_NAME.to_string()).render(window)),
            )
    }
}
