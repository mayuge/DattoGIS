use crate::apps::organisms::common::services::map::use_map_instance::MapInstance;
use crate::apps::organisms::common::services::map::use_map_tile::MapTile;
use crate::apps::organisms::common::services::map::vector_layer_service::VectorLayerService;
use crate::domain::traits::map_tile_trait::MapTileTrait;
use crate::domain::traits::vector_layer_service_trait::{ScreenGeometry, VectorLayerServiceTrait};
use crate::domain::types::map_layer_type::{VectorFeature, VectorLayer, VectorStyle};

pub type VectorGeometryCacheEntry = (VectorStyle, f32, Vec<ScreenGeometry>);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GeometryCacheKey {
    pub zoom_level: u32,
    pub center_x: i64,
    pub center_y: i64,
    pub viewport_width: i32,
    pub viewport_height: i32,
}

/// 表示用の計算をまとめる純粋なサービス。
///
/// UIコンポーネントはこのサービスを呼び出して最終的な描画情報だけを受け取り、
/// 実際のレンダリングに集中する。
pub struct MapRenderService;

impl MapRenderService {
    /// 地図の状態が更新されたかどうかを判定するためのキャッシュキーを生成する。
    pub fn build_geometry_cache_key(
        map: &MapInstance,
        viewport_width: f32,
        viewport_height: f32,
    ) -> GeometryCacheKey {
        let zoom_level = map.zoom_level.round() as u32;
        let center_x = map.center.x.round() as i64;
        let center_y = map.center.y.round() as i64;

        GeometryCacheKey {
            zoom_level,
            center_x,
            center_y,
            viewport_width: viewport_width.round() as i32,
            viewport_height: viewport_height.round() as i32,
        }
    }

    /// 現在の地図状態から、画面に必要なラスタータイルを計算する。
    pub fn resolve_visible_tiles(
        map: &MapInstance,
        viewport_width: f32,
        viewport_height: f32,
    ) -> Vec<MapTile> {
        MapTile::calculate_visible_tiles(map, viewport_width, viewport_height)
    }

    /// 現在の表示範囲とレイヤー状態から、ベクターレイヤーの描画用座標を計算する。
    pub fn resolve_vector_geometries(
        vector_layers: &[(VectorLayer, Vec<VectorFeature>)],
        map: &MapInstance,
        viewport_center: (f64, f64),
    ) -> Vec<VectorGeometryCacheEntry> {
        let service = VectorLayerService;
        let zoom_level = map.zoom_level.round() as u32;

        vector_layers
            .iter()
            .filter(|(layer, _)| layer.visible)
            .map(|(layer, features)| {
                (
                    layer.style.clone(),
                    layer.opacity,
                    service.screen_geometries(features, map.center, zoom_level, viewport_center),
                )
            })
            .collect()
    }
}
