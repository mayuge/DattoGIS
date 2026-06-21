//! タイル座標変換とURL生成ロジック

use super::use_map_instance::{Coordinate, MapInstance};

/// タイルURL生成の trait
pub trait TileUrlGenerator {
    /// タイルURLテンプレートの `{z}`、`{x}`、`{y}` を置換して返す
    ///
    /// # Arguments
    ///
    /// * `tile_url` - 生成に使うタイルURLテンプレート
    fn generate_tile_url(&self, tile_url: &str) -> String;
}

impl TileUrlGenerator for MapInstance {
    fn generate_tile_url(&self, tile_url: &str) -> String {
        let rounded_zoom_level = self.zoom_level.round().max(0.0) as u32;
        let (tile_x, tile_y) = convert_tile_from_coordinate(&self.center, rounded_zoom_level);

        tile_url
            .replace("{z}", &rounded_zoom_level.to_string())
            .replace("{x}", &tile_x.to_string())
            .replace("{y}", &tile_y.to_string())
    }
}

/// 座標からWeb Mercatorタイルインデックスを計算する
///
/// # Arguments
///
/// * `center` - 中心座標
/// * `zoom_level` - ズームレベル
///
/// # Returns
///
/// タイルX座標とY座標のタプル
fn convert_tile_from_coordinate(center: &Coordinate, zoom_level: u32) -> (u32, u32) {
    // Web Mercator のタイル座標は経度を [0, 360] 度に変換し、
    // 緯度は正規化した値を扱う必要がある
    let normalized_longitude = center.longitude.clamp(-180.0, 180.0);
    let normalized_latitude = center.latitude.clamp(-85.05112878, 85.05112878);
    let tile_count = 2_u32.pow(zoom_level);

    // 経度からタイルX座標を計算
    let tile_x = ((normalized_longitude + 180.0) / 360.0 * tile_count as f64).floor();
    let tile_x = if tile_x < 0.0 {
        0
    } else if tile_x >= tile_count as f64 {
        tile_count - 1
    } else {
        tile_x as u32
    };

    // 緯度からタイルY座標を計算（メルカトル図法）
    let latitude_radians = normalized_latitude.to_radians();
    let tile_y = ((1.0
        - (latitude_radians.tan() + 1.0 / latitude_radians.cos()).ln() / std::f64::consts::PI)
        / 2.0
        * tile_count as f64)
        .floor();
    let tile_y = if tile_y < 0.0 {
        0
    } else if tile_y >= tile_count as f64 {
        tile_count - 1
    } else {
        tile_y as u32
    };

    (tile_x, tile_y)
}

