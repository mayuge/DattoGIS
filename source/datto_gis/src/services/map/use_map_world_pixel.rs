use crate::domain::params::map_config::{RASTER_TILE_SIZE, WEB_MERCATOR_HALF_WORLD_WIDTH};
use crate::domain::traits::world_pixel_trait::WorldPixelTrait;
use crate::domain::types::map_coordinate::WebMercatorCoordinate;

#[derive(Debug, Clone, Copy)]
pub struct WorldPixel {
    pub pixel_x: f64,
    pub pixel_y: f64,
}

impl WorldPixelTrait for WorldPixel {
    fn convert_coordinate_to_pixel(coordinate: &WebMercatorCoordinate, zoom_level: u32) -> Self {
        let world_size = RASTER_TILE_SIZE * (1u32 << zoom_level) as f64;
        let world_width = WEB_MERCATOR_HALF_WORLD_WIDTH * 2.0;

        let pixel_x = (coordinate.x + WEB_MERCATOR_HALF_WORLD_WIDTH) / world_width * world_size;
        let pixel_y = (WEB_MERCATOR_HALF_WORLD_WIDTH - coordinate.y) / world_width * world_size;

        Self { pixel_x, pixel_y }
    }

    fn tile_column(&self) -> u32 {
        (self.pixel_x / RASTER_TILE_SIZE).floor() as u32
    }

    fn tile_row(&self) -> u32 {
        (self.pixel_y / RASTER_TILE_SIZE).floor() as u32
    }

    fn pixel_offset_x(&self) -> f64 {
        self.pixel_x % RASTER_TILE_SIZE
    }

    fn pixel_offset_y(&self) -> f64 {
        self.pixel_y % RASTER_TILE_SIZE
    }
}
