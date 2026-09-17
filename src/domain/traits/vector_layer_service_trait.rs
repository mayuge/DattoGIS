use crate::domain::types::map_coordinate_type::WebMercatorCoordinate;
use crate::domain::types::map_layer_type::VectorFeature;

#[derive(Clone, Debug)]
pub enum ScreenGeometry {
    Point((f32, f32)),
    LineString(Vec<(f32, f32)>),
}

pub trait VectorLayerServiceTrait {
    fn screen_geometries(
        &self,
        features: &[VectorFeature],
        center: WebMercatorCoordinate,
        zoom_level: u32,
        viewport_center: (f64, f64),
    ) -> Vec<ScreenGeometry>;
}