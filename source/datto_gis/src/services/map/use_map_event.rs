use crate::domain::map_config::{RASTER_TILE_SIZE, WEB_MERCATOR_HALF_WORLD_WIDTH};
use crate::domain::traits::map_event_trait::MapEventTrait;
use crate::services::map::use_map_instance::MapInstance;

pub struct MapEvent;

impl MapEventTrait for MapEvent {
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
        let meters_per_pixel = WEB_MERCATOR_HALF_WORLD_WIDTH * 2.0
            / (RASTER_TILE_SIZE * (1u32 << zoom_level) as f64);

        map.center.x = (map.center.x - f64::from(delta_x) * meters_per_pixel)
            .clamp(-WEB_MERCATOR_HALF_WORLD_WIDTH, WEB_MERCATOR_HALF_WORLD_WIDTH);
        map.center.y = (map.center.y + f64::from(delta_y) * meters_per_pixel)
            .clamp(-WEB_MERCATOR_HALF_WORLD_WIDTH, WEB_MERCATOR_HALF_WORLD_WIDTH);
    }
}
