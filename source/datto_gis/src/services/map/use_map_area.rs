use crate::domain::design_token_config::{HEADER_HEIGHT, LAYER_CONTROLLER_WIDTH};
use crate::domain::traits::map_area::MapAreaUseCase;

/// 地図表示領域のサイズを管理する
#[derive(Debug, Clone, Copy)]
pub struct MapArea {
    pub width: f32,
    pub height: f32,
}

impl MapAreaUseCase for MapArea {
    /// 地図表示領域のサイズを計算する
    fn get_map_area_size(window_width: f32, window_height: f32) -> Self {
        Self {
            width: (window_width - LAYER_CONTROLLER_WIDTH).max(0.0),
            height: (window_height - HEADER_HEIGHT).max(0.0),
        }
    }

    /// 地図表示領域の中心座標を計算する
    fn get_map_area_center(self) -> (f32, f32) {
        (
            LAYER_CONTROLLER_WIDTH + self.width / 2.0,
            HEADER_HEIGHT + self.height / 2.0,
        )
    }
}
