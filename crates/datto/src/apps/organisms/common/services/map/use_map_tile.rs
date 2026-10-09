use std::error::Error;

use crate::domain::params::map_config::RASTER_TILE_OVERSCAN;
use crate::domain::traits::map_tile_trait::MapTileTrait;

use super::use_map_engine;
use super::use_map_instance::MapInstance;

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

impl MapTileTrait for MapTile {
    /// タイルURLを生成する
    /// URL テンプレートの座標プレースホルダーをタイル値で置換する。
    fn generate_tile_url(&self, tile_url: &str) -> String {
        tile_url
            .replace("{z}", &self.zoom_level.to_string())
            .replace("{x}", &self.tile_column.to_string())
            .replace("{y}", &self.tile_row.to_string())
    }
    /// 表示するラスタータイル一覧を計算する
    /// マップインスタンス（xyz）、画面の幅、高さが渡される
    /// ビューポートに描画すべきタイルと画面上の座標を算出する。
    fn calculate_visible_tiles(
        map: &MapInstance,
        viewport_width: f32,
        viewport_height: f32,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        if !viewport_width.is_finite()
            || !viewport_height.is_finite()
            || viewport_width <= 0.0
            || viewport_height <= 0.0
        {
            return Err("Invalid viewport dimensions".into());
        }

        let map_obj = use_map_engine::create_map_obj_with_overscan(
            map,
            f64::from(viewport_width),
            f64::from(viewport_height),
            u32::try_from(RASTER_TILE_OVERSCAN)?,
        )?;

        Ok(map_obj
            .tiles
            .into_iter()
            .map(|tile| Self {
                tile_column: tile.x,
                tile_row: tile.y,
                zoom_level: tile.z,
                draw_x: tile.delta[0] as f32,
                draw_y: tile.delta[1] as f32,
            })
            .collect())
    }
}
