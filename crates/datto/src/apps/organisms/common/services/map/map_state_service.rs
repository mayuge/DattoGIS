use crate::apps::organisms::common::services::map::use_map_bbox::MapBbox;
use crate::apps::organisms::common::services::map::use_map_event::MapEvent;
use crate::apps::organisms::common::services::map::use_map_instance::MapInstance;
use crate::domain::traits::load_vector_json_trait::LoadVectorJsonTrait;
use crate::domain::traits::map_event_trait::MapEventTrait;
use crate::domain::traits::vector_repository_trait::VectorRepositoryTrait;
use crate::domain::types::map_layer_type::{VectorFeature, VectorLayer};
use crate::infrastructure::duckdb::vector_repository::DuckDbVectorRepository;
use crate::infrastructure::json::load_vector_json::LoadVectorJson;

/// 地図の状態遷移や再計算に関する純粋なロジックをまとめるサービス。
pub struct MapStateService;

impl MapStateService {
    /// スクロールによるズーム更新を適用する。
    pub fn zoom(map: &mut MapInstance, delta_y: f32) {
        MapEvent::zoom_by_scroll(map, delta_y);
    }

    /// ドラッグによる平行移動を適用する。
    pub fn pan(map: &mut MapInstance, delta_x: f32, delta_y: f32) {
        MapEvent::pan_by_pixels(map, delta_x, delta_y);
    }

    /// 現在の表示範囲に合わせてベクターレイヤーを再読み込みする。
    pub fn reload_vector_layers(
        repository: &DuckDbVectorRepository,
        map: &MapInstance,
        viewport_width: f32,
        viewport_height: f32,
    ) -> Vec<(VectorLayer, Vec<VectorFeature>)> {
        let bbox = if viewport_width > 0.0 && viewport_height > 0.0 {
            Some(MapBbox::calculate(map, viewport_width, viewport_height))
        } else {
            None
        };

        LoadVectorJson::load()
            .into_iter()
            .filter_map(|layer| {
                let result = match bbox {
                    Some(bbox) => repository.load_features_in_bbox(&layer, bbox),
                    None => repository.load_features(&layer),
                };

                match result {
                    Ok(features) => Some((layer, features)),
                    Err(err) => {
                        eprintln!("failed to reload vector layer: {err:#}");
                        None
                    }
                }
            })
            .collect()
    }
}
