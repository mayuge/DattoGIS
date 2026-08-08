use crate::domain::params::app_config::*;
use crate::domain::params::map_config::DEFAULT_RASTER_TILE_URL;
use crate::domain::traits::coordinate_transformer_trait::CoordinateTransformer;
use crate::domain::traits::load_raster_tile_config_trait::LoadRasterTileConfigTrait;
use crate::domain::traits::map_area_trait::MapAreaTrait;
use crate::domain::types::map_coordinate_type::EpsgCoordinate;
use crate::domain::types::map_layer_type::RasterTileLayer;
use crate::infrastructure::coordinate::proj_core_coordinate_transformer::ProjCoreCoordinateTransformer;
use crate::infrastructure::json::load_raster_tile_config::LoadRasterTileConfig;
use crate::services::map::use_map_area::MapArea;
use crate::services::map::use_map_instance::MapInstance;
use gpui::*;

use crate::apps::organisms::main_window::activity_bar_app::ActivityBarApp;
use crate::apps::organisms::main_window::layer_controller_app::LayerControllerApp;
use crate::apps::organisms::main_window::map::map_app::MapApp;

use crate::components::atoms::footer::Footer;
use crate::components::atoms::header::Header;
use crate::components::atoms::search_input::SearchInput;
use std::path::PathBuf;

//main_templateは、地図の初期状態とアプリ全体のUI構造を定義する
pub struct MainTemplate {
    search_input: Entity<SearchInput>,
    pub map: MapInstance,
    pub raster_tile_layers: Vec<RasterTileLayer>,
}

use crate::domain::params::map_config::{
    DATA_PROJ_EPSG, MAP_CENTER_LATITUDE, MAP_CENTER_LONGITUDE,
};

impl MainTemplate {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let transformer = ProjCoreCoordinateTransformer;
        let map_center = transformer
            .epsg_coordinate_to_web_mercator(EpsgCoordinate {
                x: MAP_CENTER_LONGITUDE,
                y: MAP_CENTER_LATITUDE,
                epsg: DATA_PROJ_EPSG,
            })
            .expect("failed to convert initial map center to Web Mercator");

        Self {
            search_input: cx.new(|cx| SearchInput::new(window, cx)),
            map: MapInstance::new(map_center),
            raster_tile_layers: LoadRasterTileConfig::load(),
        }
    }
}

impl Render for MainTemplate {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let viewport = window.viewport_size();

        let map_viewport =
            MapArea::get_map_area_size(f32::from(viewport.width), f32::from(viewport.height));

        let (crosshair_x, crosshair_y) = map_viewport.get_map_area_center();

        div()
            .relative()
            .size_full()
            .child(div().absolute().inset_0().child(MapApp::render(
                window,
                cx,
                self.map.clone(),
                self.raster_tile_layers.clone(),
            )))
            .child(LayerControllerApp::render(window, cx))
            .child(ActivityBarApp::render())
            .child(Header::new(APP_NAME.to_string()).render())
            .child(
                div()
                    .flex()
                    .justify_center()
                    .child(self.search_input.clone()),
            )
            .child(Footer::new(self.map.clone()).render())
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
            .into_any()
    }
}
