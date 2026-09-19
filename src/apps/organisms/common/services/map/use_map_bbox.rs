use crate::apps::organisms::common::services::map::use_map_instance::MapInstance;
use crate::apps::organisms::common::services::map::use_map_world_pixel::WorldPixel;
use crate::domain::params::map_config::WEB_MERCATOR_HALF_WORLD_WIDTH;
use crate::domain::traits::coordinate_transformer_trait::CoordinateTransformer;
use crate::domain::traits::world_pixel_trait::WorldPixelTrait;
use crate::domain::types::map_coordinate_type::WebMercatorCoordinate;
use crate::infrastructure::coordinate::proj_core_coordinate_transformer::ProjCoreCoordinateTransformer;

pub struct MapBbox;

impl MapBbox {
    pub fn calculate(map: &MapInstance, width: f32, height: f32) -> (f64, f64, f64, f64) {
        let zoom_level = map.zoom_level.round() as u32;
        let center = WorldPixel::convert_coordinate_to_pixel(&map.center, zoom_level);
        let world_width = WEB_MERCATOR_HALF_WORLD_WIDTH * 2.0;
        let world_size = 256.0 * (1u32 << zoom_level) as f64;
        let min_x = center.pixel_x - f64::from(width) / 2.0;
        let max_x = center.pixel_x + f64::from(width) / 2.0;
        let min_y = center.pixel_y - f64::from(height) / 2.0;
        let max_y = center.pixel_y + f64::from(height) / 2.0;
        let corners = [
            (min_x, min_y),
            (min_x, max_y),
            (max_x, min_y),
            (max_x, max_y),
        ];
        let coordinates: Vec<_> = corners
            .into_iter()
            .filter_map(|(x, y)| {
                ProjCoreCoordinateTransformer
                    .web_mercator_to_epsg_coordinate(
                        WebMercatorCoordinate {
                            x: (x / world_size * world_width - WEB_MERCATOR_HALF_WORLD_WIDTH)
                                .clamp(
                                    -WEB_MERCATOR_HALF_WORLD_WIDTH,
                                    WEB_MERCATOR_HALF_WORLD_WIDTH,
                                ),
                            y: (WEB_MERCATOR_HALF_WORLD_WIDTH - y / world_size * world_width)
                                .clamp(
                                    -WEB_MERCATOR_HALF_WORLD_WIDTH,
                                    WEB_MERCATOR_HALF_WORLD_WIDTH,
                                ),
                        },
                        4326,
                    )
                    .ok()
            })
            .collect();
        (
            coordinates
                .iter()
                .map(|coordinate| coordinate.x)
                .fold(f64::INFINITY, f64::min),
            coordinates
                .iter()
                .map(|coordinate| coordinate.y)
                .fold(f64::INFINITY, f64::min),
            coordinates
                .iter()
                .map(|coordinate| coordinate.x)
                .fold(f64::NEG_INFINITY, f64::max),
            coordinates
                .iter()
                .map(|coordinate| coordinate.y)
                .fold(f64::NEG_INFINITY, f64::max),
        )
    }
}
