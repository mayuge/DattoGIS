use crate::domain::map_config::{MAP_MAX_LATITUDE, MAP_MIN_LATITUDE, RASTER_TILE_SIZE};
use crate::domain::traits::map_event::MapEventUseCase;
use crate::services::map::use_map_instance::MapInstance;

/// 地図描画部分でのイベントの計算
pub struct MapEvent;

impl MapEventUseCase for MapEvent {
    fn zoom_by_scroll(map: &mut MapInstance, scroll_delta_y: f32) {
        let zoom_step = if scroll_delta_y > 0.0 {
            1.0
        } else if scroll_delta_y < 0.0 {
            -1.0
        } else {
            0.0
        };

        map.zoom_level += zoom_step;
    }

    fn pan_by_pixels(map: &mut MapInstance, delta_x: f32, delta_y: f32) {
        let zoom_level = map.zoom_level.round() as u32;
        let world_size = RASTER_TILE_SIZE * (1u32 << zoom_level) as f64;
        let pixels_per_degree_longitude = world_size / 360.0;
        let pixels_per_degree_latitude = world_size / 180.0;

        map.center.longitude = (map.center.longitude
            - f64::from(delta_x) / pixels_per_degree_longitude)
            .clamp(-180.0, 180.0);
        map.center.latitude = (map.center.latitude
            + f64::from(delta_y) / pixels_per_degree_latitude)
            .clamp(MAP_MIN_LATITUDE, MAP_MAX_LATITUDE);
    }

    fn coordinate_text(map: &MapInstance) -> String {
        format!("{:.4}, {:.4}", map.center.latitude, map.center.longitude)
    }
}
