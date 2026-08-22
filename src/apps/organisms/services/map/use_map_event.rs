use crate::domain::params::map_config::{
    MAP_MAX_ZOOM_LEVEL, MAP_MIN_ZOOM_LEVEL, RASTER_TILE_SIZE, WEB_MERCATOR_HALF_WORLD_WIDTH,
};
use crate::domain::traits::map_event_trait::MapEventTrait;
use crate::apps::organisms::services::map::use_map_instance::MapInstance;

pub struct MapEvent;

impl MapEventTrait for MapEvent {
    //マップズーム
    fn zoom_by_scroll(map: &mut MapInstance, scroll_delta_y: f32) {
        let zoom_step = if scroll_delta_y > 0.0 {
            1.0
        } else if scroll_delta_y < 0.0 {
            -1.0
        } else {
            0.0
        };

        map.zoom_level = (map.zoom_level + zoom_step).clamp(MAP_MIN_ZOOM_LEVEL, MAP_MAX_ZOOM_LEVEL);
    }

    //マップ移動
    fn pan_by_pixels(map: &mut MapInstance, delta_x: f32, delta_y: f32) {
        let zoom_level = map.zoom_level.round() as u32;
        //WEB_MERCATOR_HALF_WORLD_WIDTHは、メートル換算したWeb Mercatorの原点から東西端(円周の半分)×円周率(π)の値
        //世界全体の投影幅 2πR（約 40,075km）、WEB_MERCATOR_HALF_WORLD_WIDTH * 2.0
        //meters_per_pixelでは、RASTER_TILE_SIZE=256pxとして、ズームレベルの数だけ累乗した結果、RASTER_TILE_SIZE × 2^zoom_level
        //ズームすると増えるタイルに対して、縮尺を小さくでき、ズームレベルに合わせた移動になる
        let meters_per_pixel =
            WEB_MERCATOR_HALF_WORLD_WIDTH * 2.0 / (RASTER_TILE_SIZE * (1u32 << zoom_level) as f64);
        //メルカトル図法では、両極を表示できないため切り捨てる
        map.center.x = (map.center.x - f64::from(delta_x) * meters_per_pixel).clamp(
            -WEB_MERCATOR_HALF_WORLD_WIDTH,
            WEB_MERCATOR_HALF_WORLD_WIDTH,
        );
        //メルカトル図法では、両極を表示できないため切り捨てる
        map.center.y = (map.center.y + f64::from(delta_y) * meters_per_pixel).clamp(
            -WEB_MERCATOR_HALF_WORLD_WIDTH,
            WEB_MERCATOR_HALF_WORLD_WIDTH,
        );
    }
}
