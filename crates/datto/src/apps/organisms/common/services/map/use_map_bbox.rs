use std::error::Error;

use crate::apps::organisms::common::services::map::use_map_engine;
use crate::apps::organisms::common::services::map::use_map_instance::MapInstance;

pub struct MapBbox;

impl MapBbox {
    pub fn calculate(
        map: &MapInstance,
        width: f32,
        height: f32,
    ) -> Result<(f64, f64, f64, f64), Box<dyn Error>> {
        let map_obj = use_map_engine::create_map_obj(map, f64::from(width), f64::from(height))?;
        let [min_x, min_y, max_x, max_y] = map_obj.bbox;
        Ok((min_x, min_y, max_x, max_y))
    }
}
