use gpui::*;

use crate::domain::app_config::*;
use crate::domain::design_token_config::{HEADER_HEIGHT, LAYER_CONTROLLER_WIDTH};
use crate::domain::map_config::DEFAULT_RASTER_TILE_URL;

use crate::apps::app::App as AppState;
use crate::apps::organisms::main_window::activity_bar_app::ActivityBarApp;
use crate::apps::organisms::main_window::layer_controller_app::LayerControllerApp;
use crate::apps::organisms::main_window::map_app::MapApp;

use crate::components::atoms::footer::Footer;
use crate::components::atoms::header::Header;
use crate::components::atoms::search_input::SearchInput;

use gpui::App as GpuiApp;
use std::path::PathBuf;
use std::rc::Rc;

pub struct MainTemplate;

impl MainTemplate {
    pub fn render(window: &mut Window, cx: &mut Context<AppState>) -> impl IntoElement {
        let coordinate_display = window.use_state(cx, |_, _| SharedString::from(String::new()));
        let set_coordinate = {
            let coord_state = coordinate_display.clone();
            Rc::new(move |app: &mut GpuiApp, s: SharedString| {
                coord_state.update(app, |val, _| {
                    *val = s.clone();
                });
            })
        };
        let viewport = window.viewport_size();
        let viewport_width = f32::from(viewport.width);
        let viewport_height = f32::from(viewport.height);
        let map_viewport_width = viewport_width - LAYER_CONTROLLER_WIDTH;
        let map_viewport_height = viewport_height - HEADER_HEIGHT;

        div()
            .relative()
            .size_full()
            .child(div().absolute().inset_0().child(MapApp::render(
                window,
                cx,
                DEFAULT_RASTER_TILE_URL,
                set_coordinate.clone(),
            )))
            .child(LayerControllerApp::render(window, cx))
            .child(ActivityBarApp::render())
            .child(Header::new(APP_NAME.to_string()).render())
            .child(
                div()
                    .flex()
                    .justify_center()
                    .child(SearchInput::new().render()),
            )
            .child(Footer::new(coordinate_display.read(cx).clone()).render())
            .child(
                img(PathBuf::from("assets/map/crosshair.svg"))
                    .absolute()
                    .top(px(HEADER_HEIGHT + map_viewport_height / 2.0))
                    .left(px(LAYER_CONTROLLER_WIDTH + map_viewport_width / 2.0))
                    .w(px(32.0))
                    .h(px(32.0))
                    .ml(px(-16.0))
                    .mt(px(-16.0)),
            )
    }
}
