use crate::domain::params::map_config::{RASTER_TILE_SIZE, WEB_MERCATOR_HALF_WORLD_WIDTH};
use crate::domain::traits::world_pixel_trait::WorldPixelTrait;
use crate::domain::types::map_coordinate_type::WebMercatorCoordinate;

#[derive(Debug, Clone, Copy)]
pub struct WorldPixel {
    pub pixel_x: f64,
    pub pixel_y: f64,
}

impl WorldPixelTrait for WorldPixel {
    /// Web Mercator 座標を指定ズームのワールドピクセルへ変換する。
    fn convert_coordinate_to_pixel(coordinate: &WebMercatorCoordinate, zoom_level: u32) -> Self {
        // 256px*2^zoom_level(タイルの大きさ×枚数)
        //タイルの一辺の枚数は2のzoom_level乗,0なら1枚、5なら32枚
        let world_size = RASTER_TILE_SIZE * (1u32 << zoom_level) as f64;
        //WEB_MERCATOR_HALF_WORLD_WIDTHはπrなので2倍で直径になる
        let world_width = WEB_MERCATOR_HALF_WORLD_WIDTH * 2.0;

        let pixel_x = (coordinate.x + WEB_MERCATOR_HALF_WORLD_WIDTH) / world_width * world_size;
        let pixel_y = (WEB_MERCATOR_HALF_WORLD_WIDTH - coordinate.y) / world_width * world_size;

        Self { pixel_x, pixel_y }
    }

    /// ワールドピクセルを Web Mercator 座標へ変換する。
    fn convert_pixel_to_coordinate(
        pixel_x: f64,
        pixel_y: f64,
        zoom_level: u32,
    ) -> WebMercatorCoordinate {
        let world_size = RASTER_TILE_SIZE * (1u32 << zoom_level) as f64;

        let world_width = WEB_MERCATOR_HALF_WORLD_WIDTH * 2.0;

        let x = pixel_x / world_size * world_width - WEB_MERCATOR_HALF_WORLD_WIDTH;

        let y = WEB_MERCATOR_HALF_WORLD_WIDTH - (pixel_y / world_size * world_width);

        WebMercatorCoordinate { x, y }
    }

    /// ワールドピクセルが属するタイル列番号を返す。
    fn tile_column(&self) -> u32 {
        (self.pixel_x / RASTER_TILE_SIZE).floor() as u32
    }

    /// ワールドピクセルが属するタイル行番号を返す。
    fn tile_row(&self) -> u32 {
        (self.pixel_y / RASTER_TILE_SIZE).floor() as u32
    }

    /// タイル内の X 方向ピクセルオフセットを返す。
    fn pixel_offset_x(&self) -> f64 {
        self.pixel_x % RASTER_TILE_SIZE
    }

    /// タイル内の Y 方向ピクセルオフセットを返す。
    fn pixel_offset_y(&self) -> f64 {
        self.pixel_y % RASTER_TILE_SIZE
    }
}
