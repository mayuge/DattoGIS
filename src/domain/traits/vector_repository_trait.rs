use anyhow::Result;

use crate::domain::types::map_layer_type::{VectorFeature, VectorLayer};

pub trait VectorRepositoryTrait {
    fn load_features(&self, layer: &VectorLayer) -> Result<Vec<VectorFeature>>;
    fn load_features_in_bbox(
        &self,
        layer: &VectorLayer,
        bbox: (f64, f64, f64, f64),
    ) -> Result<Vec<VectorFeature>>;
}
