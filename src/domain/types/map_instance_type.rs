use crate::domain::params::map_config::MAP_ZOOM_LEVEL;
use crate::domain::types::map_coordinate_type::WebMercatorCoordinate;

#[derive(Debug, Clone)]
pub struct MapInstance {
    pub center: WebMercatorCoordinate,
    pub zoom_level: u32,
}

impl MapInstance {
    /// 初期位置とズームレベルを持つ地図インスタンスを生成する。
    pub fn new(center: WebMercatorCoordinate) -> Self {
        Self {
            center,
            zoom_level: MAP_ZOOM_LEVEL,
        }
    }

    /// 地図中心を更新する。
    pub fn set_center(&mut self, center: WebMercatorCoordinate) {
        self.center = center;
    }
}
