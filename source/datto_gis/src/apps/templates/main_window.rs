use gpui::*;

use crate::domain::app_config::*;

use crate::apps::app::App as AppState;
use crate::apps::organisms::main_window::layer_controller_app::LayerControllerApp;
use crate::apps::organisms::main_window::map_app::MapApp;

use crate::components::molecules::header::*;

pub struct MainTemplate;

impl MainTemplate {
    pub fn render(window: &mut Window, cx: &mut Context<AppState>) -> impl IntoElement {
        div()
            .relative()
            .size_full()
            .child(div().absolute().inset_0().child(MapApp::render(window, cx)))
            .child(LayerControllerApp::render(window, cx))
            .child(Header::new(APP_NAME.to_string()).render(window))
    }
}
