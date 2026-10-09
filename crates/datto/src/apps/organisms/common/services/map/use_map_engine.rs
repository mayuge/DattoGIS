use std::error::Error;

use crate::apps::organisms::common::services::map::use_map_instance::MapInstance;
use crate::domain::params::map_config::{DATA_PROJ_EPSG, DISPLAY_PROJ_EPSG};

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
    let (longitude, latitude) = engine::transform_coordinate(
        DISPLAY_PROJ_EPSG,
        DATA_PROJ_EPSG,
        (map.center.x, map.center.y),
    )?;

    engine::create_map_obj_with_overscan(
        engine::MapXYZ {
            latitude,
            longitude,
            zoom: map.zoom_level,
        },
        engine::Screen { width, height },
        overscan_tiles,
    )
}
