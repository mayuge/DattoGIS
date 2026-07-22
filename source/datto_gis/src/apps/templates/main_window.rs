use gpui::*;

use crate::domain::params::app_config::*;
use crate::domain::params::map_config::DEFAULT_RASTER_TILE_URL;
use crate::domain::traits::map_area_trait::MapAreaTrait;
use crate::services::map::use_map_area::MapArea;
use crate::services::map::use_map_instance::MapInstance;

use crate::apps::app::App as AppState;
use crate::apps::organisms::main_window::activity_bar_app::ActivityBarApp;
use crate::apps::organisms::main_window::layer_controller_app::LayerControllerApp;
use crate::apps::organisms::main_window::map_app::MapApp;

use crate::components::atoms::footer::Footer;
use crate::components::atoms::header::Header;
use crate::components::atoms::search_input::SearchInput;

use std::path::PathBuf;

pub struct MainTemplate;

impl MainTemplate {
    pub fn render(
        window: &mut Window,
        cx: &mut Context<AppState>,
        map: MapInstance,
    ) -> impl IntoElement {
        let viewport = window.viewport_size();
        let map_viewport =
            MapArea::get_map_area_size(f32::from(viewport.width), f32::from(viewport.height));
        // クロスヘア表示位置を取得
        let (crosshair_x, crosshair_y) = map_viewport.get_map_area_center();

        div()
            .relative()
            .size_full()
            .child(div().absolute().inset_0().child(MapApp::render(
                window,
                cx,
                DEFAULT_RASTER_TILE_URL,
                map.clone(),
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
            .child(Footer::new(map).render())
            //クロスヘアを配置
            .child(
                img(PathBuf::from("assets/map/crosshair.svg"))
                    .absolute()
                    .top(px(crosshair_y))
                    .left(px(crosshair_x))
                    .w(px(32.0))
                    .h(px(32.0))
                    .ml(px(-16.0))
                    .mt(px(-16.0)),
            )
    }
}
