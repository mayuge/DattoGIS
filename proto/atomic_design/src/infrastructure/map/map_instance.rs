//! 地図表示状態を管理するドメインモデル

use crate::domain::map_config::MapConfig;

/// 座標情報
#[derive(Debug, Clone)]
pub struct Coordinate {
    /// 経度
    pub longitude: f64,

    /// 緯度
    pub latitude: f64,
}

/// 地図の表示状態
#[derive(Debug, Clone)]
pub struct MapInstance {
    /// 地図の中心座標
    pub center: Coordinate,

    /// 地図のズームレベル
    pub zoom_level: f64,
}

impl Default for MapInstance {
    /// アプリケーション起動時の初期地図状態を生成する
    fn default() -> Self {
        Self {
            center: Coordinate {
                longitude: MapConfig::MAP_CENTER_LONGITUDE,
                latitude: MapConfig::MAP_CENTER_LATITUDE,
            },
            zoom_level: MapConfig::MAP_ZOOM_LEVEL,
        }
    }
}

impl MapInstance {
    /// 指定した値で地図状態を生成する
    ///
    /// # Arguments
    ///
    /// * `center` - 地図の中心座標
    /// * `zoom_level` - 地図のズームレベル
    pub fn new(center: Coordinate, zoom_level: f64) -> Self {
        Self { center, zoom_level }
    }

    /// 地図状態を更新する
    ///
    /// # Arguments
    ///
    /// * `center` - 更新後の中心座標
    /// * `zoom_level` - 更新後のズームレベル
    pub fn update(&mut self, center: Coordinate, zoom_level: f64) {
        self.center = center;
        self.zoom_level = zoom_level;
    }
}