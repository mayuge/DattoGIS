use crate::apps::organisms::common::services::map::use_map_instance::MapInstance;
use std::error::Error;

use crate::domain::params::map_config::{MAP_MAX_ZOOM_LEVEL, MAP_MIN_ZOOM_LEVEL};
use crate::domain::traits::map_event_trait::MapEventTrait;

pub struct MapEvent;

impl MapEventTrait for MapEvent {
    //マップズーム
    /// スクロール量に応じてズームレベルを一段階更新する。
    fn zoom_by_scroll(map: &mut MapInstance, scroll_delta_y: f32) {
        map.zoom_level = if scroll_delta_y > 0.0 {
            map.zoom_level.saturating_add(1)
        } else if scroll_delta_y < 0.0 {
            map.zoom_level.saturating_sub(1)
        } else {
            map.zoom_level
        };
        map.zoom_level = map.zoom_level.clamp(MAP_MIN_ZOOM_LEVEL, MAP_MAX_ZOOM_LEVEL);
    }

    //マップ移動
    /// ドラッグしたピクセル量に応じて地図中心を移動する。
    fn pan_by_pixels(
        map: &mut MapInstance,
        delta_x: f32,
        delta_y: f32,
    ) -> Result<(), Box<dyn Error>> {
        let (x, y) = engine::pan_web_mercator_center(
            (map.center.x, map.center.y),
            map.zoom_level,
            (f64::from(delta_x), f64::from(delta_y)),
        )?;
        map.center.x = x;
        map.center.y = y;
        Ok(())
    }
}
