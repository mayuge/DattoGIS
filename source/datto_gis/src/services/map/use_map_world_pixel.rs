//! Web Mercator 座標変換

use super::use_map_instance::Coordinate;

use crate::domain::map_config::{MAP_MAX_LATITUDE, MAP_MIN_LATITUDE, RASTER_TILE_SIZE};
use crate::domain::traits::world_pixel::WorldPixelUseCase;

#[derive(Debug, Clone, Copy)]
pub struct WorldPixel {
    pub pixel_x: f64,
    pub pixel_y: f64,
}

impl WorldPixelUseCase for WorldPixel {
    fn from_coordinate(coordinate: &Coordinate, zoom_level: u32) -> Self {
        let longitude = coordinate.longitude.clamp(-180.0, 180.0);
        let latitude = coordinate
            .latitude
            .clamp(MAP_MIN_LATITUDE, MAP_MAX_LATITUDE);

        let world_size = RASTER_TILE_SIZE as f64 * (1u32 << zoom_level) as f64;

        let pixel_x = (longitude + 180.0) / 360.0 * world_size;

        let latitude_radians = latitude.to_radians();

        let pixel_y = (1.0
            - (latitude_radians.tan() + 1.0 / latitude_radians.cos()).ln() / std::f64::consts::PI)
            / 2.0
            * world_size;

        Self { pixel_x, pixel_y }
    }

    fn tile_column(&self) -> u32 {
        (self.pixel_x / RASTER_TILE_SIZE as f64).floor() as u32
    }

    fn tile_row(&self) -> u32 {
        (self.pixel_y / RASTER_TILE_SIZE as f64).floor() as u32
    }

    fn pixel_offset_x(&self) -> f64 {
        self.pixel_x % RASTER_TILE_SIZE as f64
    }

    fn pixel_offset_y(&self) -> f64 {
        self.pixel_y % RASTER_TILE_SIZE as f64
    }
}
