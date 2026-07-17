//! 地図表示状態を管理するドメインモデル

use crate::domain::map_config;

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
                longitude: map_config::MAP_CENTER_LONGITUDE,
                latitude: map_config::MAP_CENTER_LATITUDE,
            },
            zoom_level: map_config::MAP_ZOOM_LEVEL,
        }
    }
}
