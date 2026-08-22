use crate::domain::types::map_bbox_type::Bbox;
use crate::domain::types::map_coordinate_type::WebMercatorCoordinate;
use crate::apps::organisms::services::map::use_map_instance::MapInstance;
use crate::apps::organisms::services::map::use_map_world_pixel::WorldPixel;

use crate::domain::traits::world_pixel_trait::WorldPixelTrait;

pub struct MapBbox;

impl MapBbox {
    pub fn calculate(map: &MapInstance, width: f32, height: f32) -> Bbox {
        let zoom_level = map.zoom_level.round() as u32;

        // 中心座標をworld pixelへ変換
        let center_pixel = WorldPixel::convert_coordinate_to_pixel(&map.center, zoom_level);

        let center_x = center_pixel.x();
        let center_y = center_pixel.y();

        // 画面左上pixel
        let min_pixel_x = center_x - width as f64 / 2.0;

        let min_pixel_y = center_y - height as f64 / 2.0;

        // 画面右下pixel
        let max_pixel_x = center_x + width as f64 / 2.0;

        let max_pixel_y = center_y + height as f64 / 2.0;

        // pixel -> WebMercator
        let min = WorldPixel::convert_pixel_to_coordinate(min_pixel_x, min_pixel_y, zoom_level);

        let max = WorldPixel::convert_pixel_to_coordinate(max_pixel_x, max_pixel_y, zoom_level);

        Bbox {
            minx: min.x,
            miny: min.y,
            maxx: max.x,
            maxy: max.y,
        }
    }
}
