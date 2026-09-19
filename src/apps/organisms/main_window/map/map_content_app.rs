use gpui::*;

use crate::apps::organisms::common::services::map::use_map_area::MapArea;
use crate::apps::organisms::common::services::map::use_map_instance::MapInstance;
use crate::apps::organisms::common::services::map::use_map_tile::MapTile;
use crate::apps::organisms::common::services::map::vector_layer_service::VectorLayerService;
use crate::apps::organisms::main_window::map::raster_tile_layer_app::RasterTileLayerApp;
use crate::apps::organisms::main_window::map::vector_layer_app::VectorLayerApp;
use crate::domain::params::design_token_config::{HEADER_HEIGHT, LAYER_CONTROLLER_WIDTH};
use crate::domain::traits::map_area_trait::MapAreaTrait;
use crate::domain::traits::map_tile_trait::MapTileTrait;
use crate::domain::traits::vector_layer_service_trait::VectorLayerServiceTrait;
use crate::domain::types::map_layer_type::{RasterTileLayer, VectorFeature, VectorLayer};

pub struct MapContent {
    map: MapInstance,
    raster_layers: Vec<RasterTileLayer>,
    vector_layers: Vec<(VectorLayer, Vec<VectorFeature>)>,
}

impl MapContent {
    pub fn new(
        map: MapInstance,
        raster_layers: Vec<RasterTileLayer>,
        vector_layers: Vec<(VectorLayer, Vec<VectorFeature>)>,
    ) -> Self {
        Self {
            map,
            raster_layers,
            vector_layers,
        }
    }

    pub fn update_state(
        &mut self,
        map: MapInstance,
        raster_layers: Vec<RasterTileLayer>,
        vector_layers: Vec<(VectorLayer, Vec<VectorFeature>)>,
        cx: &mut Context<Self>,
    ) {
        self.map = map;
        self.raster_layers = raster_layers;
        self.vector_layers = vector_layers;
        cx.notify();
    }
}

impl Render for MapContent {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let viewport = window.viewport_size();
        let map_area =
            MapArea::get_map_area_size(f32::from(viewport.width), f32::from(viewport.height));
        let visible_tiles =
            MapTile::calculate_visible_tiles(&self.map, map_area.width, map_area.height);
        let center_x = f64::from(LAYER_CONTROLLER_WIDTH) + f64::from(map_area.width) / 2.0;
        let center_y = f64::from(HEADER_HEIGHT) + f64::from(map_area.height) / 2.0;
        let service = VectorLayerService;
        let zoom_level = self.map.zoom_level.round() as u32;
        let vector_geometries = self
            .vector_layers
            .iter()
            .filter(|(layer, _)| layer.visible)
            .map(|(layer, features)| {
                (
                    layer.style.clone(),
                    layer.opacity,
                    service.screen_geometries(
                        features,
                        self.map.center,
                        zoom_level,
                        (center_x, center_y),
                    ),
                )
            })
            .collect();

        div()
            .absolute()
            .size_full()
            .child(RasterTileLayerApp::render(
                visible_tiles,
                self.raster_layers.clone(),
            ))
            .child(VectorLayerApp::render(vector_geometries))
    }
}
