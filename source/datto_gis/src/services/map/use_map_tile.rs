use super::use_map_instance::MapInstance;
use super::use_map_world_pixel::WorldPixel;

use crate::domain::map_config::{RASTER_TILE_OVERSCAN, RASTER_TILE_SIZE};

/// 描画するラスタータイル情報
#[derive(Debug, Clone)]
pub struct MapTile {
    /// タイル列番号
    pub tile_column: u32,

    /// タイル行番号
    pub tile_row: u32,

    /// ズームレベル
    pub zoom_level: u32,

    /// 描画位置X座標
    pub draw_x: f32,

    /// 描画位置Y座標
    pub draw_y: f32,
}

impl MapTile {
    /// 表示するラスタータイル一覧を計算する
    pub fn calculate_visible_tiles(
        map: &MapInstance,
        viewport_width: f32,
        viewport_height: f32,
    ) -> Vec<Self> {
        let zoom_level = map.zoom_level.round() as u32;

        let world_pixel = WorldPixel::from_coordinate(&map.center, zoom_level);

        let center_tile_column = world_pixel.tile_column() as i32;
        let center_tile_row = world_pixel.tile_row() as i32;

        let pixel_offset_x = world_pixel.pixel_offset_x() as f32;
        let pixel_offset_y = world_pixel.pixel_offset_y() as f32;

        // 画面に必要なタイル枚数（余白を追加）
        let visible_tile_count_x =
            (viewport_width / RASTER_TILE_SIZE as f32).ceil() as i32 + RASTER_TILE_OVERSCAN * 2;

        let visible_tile_count_y =
            (viewport_height / RASTER_TILE_SIZE as f32).ceil() as i32 + RASTER_TILE_OVERSCAN * 2;

        let mut visible_tiles = Vec::new();

        for row_offset in -visible_tile_count_y / 2..=visible_tile_count_y / 2 {
            for column_offset in -visible_tile_count_x / 2..=visible_tile_count_x / 2 {
                let tile_column = center_tile_column + column_offset;
                let tile_row = center_tile_row + row_offset;

                if tile_column < 0 || tile_row < 0 {
                    continue;
                }

                let draw_x = viewport_width / 2.0 - pixel_offset_x
                    + column_offset as f32 * RASTER_TILE_SIZE as f32;

                let draw_y = viewport_height / 2.0 - pixel_offset_y
                    + row_offset as f32 * RASTER_TILE_SIZE as f32;

                visible_tiles.push(Self {
                    tile_column: tile_column as u32,
                    tile_row: tile_row as u32,
                    zoom_level,
                    draw_x,
                    draw_y,
                });
            }
        }

        visible_tiles
    }
}
