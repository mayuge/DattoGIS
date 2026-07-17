//! Web Mercator 座標変換

use super::use_map_instance::Coordinate;

use crate::domain::map_config::{MAP_MAX_LATITUDE, MAP_MIN_LATITUDE, RASTER_TILE_SIZE};
use crate::domain::traits::world_pixel_trait::WorldPixelTrait;

#[derive(Debug, Clone, Copy)]
pub struct WorldPixel {
    pub pixel_x: f64,
    pub pixel_y: f64,
}

impl WorldPixelTrait for WorldPixel {
    //1タイル256pxとして、緯度経度ズームレベルをピクセルに変換する
    fn convert_coordinate_to_pixel(coordinate: &Coordinate, zoom_level: u32) -> Self {
        //経度
        let longitude = coordinate.longitude.clamp(-180.0, 180.0);

        //緯度
        let latitude = coordinate
            .latitude
            .clamp(MAP_MIN_LATITUDE, MAP_MAX_LATITUDE); //メルカトル図法では、1/cos90°の時に、無限大になるため絶対値で制限をかける必要がある

        //タイルサイズ（256px）として、ズームレベルで累乗する
        let world_size = RASTER_TILE_SIZE as f64 * (1u32 << zoom_level) as f64;

        //0から360の範囲に戻す
        let pixel_x = (longitude + 180.0) / 360.0 * world_size;

        let latitude_radians = latitude.to_radians();

        let pixel_y = (1.0
            - (latitude_radians.tan() + 1.0 / latitude_radians.cos()).ln() / std::f64::consts::PI)
            / 2.0
            * world_size;

        Self { pixel_x, pixel_y }
    }
    //タイルを敷き詰める縦位置
    fn tile_column(&self) -> u32 {
        (self.pixel_x / RASTER_TILE_SIZE as f64).floor() as u32
    }
    //タイルを敷き詰める横位置
    fn tile_row(&self) -> u32 {
        (self.pixel_y / RASTER_TILE_SIZE as f64).floor() as u32
    }
    //256pxで割ったときの余りで、横位置をオフセットする
    fn pixel_offset_x(&self) -> f64 {
        self.pixel_x % RASTER_TILE_SIZE as f64
    }
    //256pxで割ったときの余りで、縦位置をオフセットする
    fn pixel_offset_y(&self) -> f64 {
        self.pixel_y % RASTER_TILE_SIZE as f64
    }
}
