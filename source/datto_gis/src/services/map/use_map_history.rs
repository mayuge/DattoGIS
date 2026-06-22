use geojson::Feature as GeoJsonFeature;
use std::sync::Arc;

/// DattoGIS内部ID
pub type FeatureId = u64;

/// GISフィーチャ
#[derive(Debug, Clone)]
pub struct Feature {
    pub id: FeatureId,
    pub geojson: GeoJsonFeature,
}

impl Feature {
    pub fn new(id: FeatureId, geojson: GeoJsonFeature) -> Self {
        Self { id, geojson }
    }
}

/// 編集履歴
#[derive(Debug, Clone)]
pub enum ChangeCategory {
    Created {
        id: FeatureId,
        after: Arc<Feature>,
    },

    Deleted {
        id: FeatureId,
        before: Arc<Feature>,
    },

    Updated {
        id: FeatureId,
        before: Arc<Feature>,
        after: Arc<Feature>,
    },
}

/// Undo/Redo履歴
#[derive(Debug, Default)]
pub struct MapHistory {
    pub changes: Vec<ChangeCategory>,
}

impl MapHistory {
    pub fn new() -> Self {
        Self {
            changes: Vec::new(),
        }
    }
}
