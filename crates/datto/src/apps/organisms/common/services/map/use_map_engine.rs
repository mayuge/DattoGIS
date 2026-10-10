use std::error::Error;

use crate::apps::organisms::common::services::map::use_map_instance::MapInstance;
pub fn create_map_obj(
    map: &MapInstance,
    width: f64,
    height: f64,
) -> Result<engine::MapObject, Box<dyn Error>> {
    create_map_obj_with_overscan(map, width, height, 0)
}

pub fn create_map_obj_with_overscan(
    map: &MapInstance,
    width: f64,
    height: f64,
    overscan_tiles: u32,
) -> Result<engine::MapObject, Box<dyn Error>> {
    map.engine_view()
        .create_map_obj(engine::Screen { width, height }, overscan_tiles)
}
