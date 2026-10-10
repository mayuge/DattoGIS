use crate::domain::types::map_coordinate_type::WebMercatorCoordinate;
use crate::domain::types::map_layer_type::VectorFeature;
use vector_engine::{ScreenGeometry, VectorGeometryProjector};

pub struct VectorLayerService;

impl VectorLayerService {
    pub fn screen_geometries(
        features: &[VectorFeature],
        epsg: u32,
        center: WebMercatorCoordinate,
        zoom_level: u32,
        viewport_center: (f64, f64),
        projector: &dyn VectorGeometryProjector,
    ) -> Vec<ScreenGeometry> {
        let transform = match engine::ViewportTransform::new(
            (center.x, center.y),
            zoom_level,
            viewport_center,
            epsg,
        ) {
            Ok(transform) => transform,
            Err(error) => {
                eprintln!("failed to create vector viewport transform: {error}");
                return Vec::new();
            }
        };
        let wkb = features
            .iter()
            .map(|feature| feature.geometry_wkb.as_slice())
            .collect::<Vec<_>>();

        projector
            .project_wkb_features(&wkb, &transform)
            .into_iter()
            .flat_map(|result| match result {
                Ok(geometries) => geometries,
                Err(error) => {
                    eprintln!("failed to decode or project vector geometry: {error}");
                    Vec::new()
                }
            })
            .collect()
    }
}
