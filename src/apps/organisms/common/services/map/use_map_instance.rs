use crate::domain::params::map_config::MAP_ZOOM_LEVEL;
use crate::domain::types::map_coordinate_type::WebMercatorCoordinate;

#[derive(Debug, Clone)]
pub struct MapInstance {
    pub center: WebMercatorCoordinate,
    pub zoom_level: f64,
}

impl MapInstance {
    /// 指定した中心座標で地図状態を生成する。
    pub fn new(center: WebMercatorCoordinate) -> Self {
        Self {
            center,
            zoom_level: MAP_ZOOM_LEVEL,
        }
    }
}
