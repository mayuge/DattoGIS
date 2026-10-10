use crate::domain::params::map_config::MAP_ZOOM_LEVEL;
use crate::domain::types::map_coordinate_type::WebMercatorCoordinate;

#[derive(Clone)]
pub struct MapInstance {
    view: engine::MapView,
}

impl MapInstance {
    /// 初期位置とズームレベルを持つ地図インスタンスを生成する。
    pub fn new(center: WebMercatorCoordinate) -> Self {
        Self {
            view: engine::MapView::new((center.x, center.y), MAP_ZOOM_LEVEL)
                .expect("invalid initial map view"),
        }
    }

    pub fn center(&self) -> WebMercatorCoordinate {
        let (x, y) = self.view.center();
        WebMercatorCoordinate { x, y }
    }

    pub fn zoom_level(&self) -> u32 {
        self.view.zoom()
    }

    /// 地図中心を更新する。
    pub fn set_center(&mut self, center: WebMercatorCoordinate) {
        self.view.set_center((center.x, center.y));
    }

    pub fn set_zoom_level(&mut self, zoom_level: u32) {
        self.view
            .set_zoom(zoom_level)
            .expect("invalid map zoom level");
    }

    pub fn move_xy(&mut self, movement: engine::MoveXY) -> Result<(), Box<dyn std::error::Error>> {
        self.view.move_xy(movement)
    }

    pub fn engine_view(&self) -> &engine::MapView {
        &self.view
    }
}
