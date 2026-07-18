use crate::domain::map_config::MAP_ZOOM_LEVEL;
use crate::domain::map_coordinate::WebMercatorCoordinate;

#[derive(Debug, Clone)]
pub struct MapInstance {
    pub center: WebMercatorCoordinate,
    pub zoom_level: f64,
}

impl MapInstance {
    pub fn new(center: WebMercatorCoordinate) -> Self {
        Self {
            center,
            zoom_level: MAP_ZOOM_LEVEL,
        }
    }
}
