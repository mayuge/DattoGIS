use crate::apps::organisms::common::services::map::use_map_instance::MapInstance;

pub trait MapTileTrait: Sized {
    /// タイル URL テンプレートを具体的な URL に展開する。
    fn generate_tile_url(&self, tile_url: &str) -> String;
    /// 現在の地図表示範囲に必要なタイルを計算する。
    fn calculate_visible_tiles(
        map: &MapInstance,
        viewport_width: f32,
        viewport_height: f32,
    ) -> Vec<Self>;
}
