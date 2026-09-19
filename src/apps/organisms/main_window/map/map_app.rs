use gpui::*;

use crate::apps::organisms::common::services::map::map_render_service::{
    GeometryCacheKey, MapRenderService, VectorGeometryCacheEntry,
};
use crate::apps::organisms::common::services::map::map_state_service::MapStateService;
use crate::apps::organisms::common::services::map::use_map_area::MapArea;
use crate::apps::organisms::common::services::map::use_map_bbox::MapBbox;
use crate::apps::organisms::common::services::map::use_map_instance::MapInstance;
use crate::apps::organisms::common::services::map::use_map_tile::MapTile;
use crate::apps::organisms::main_window::map::raster_tile_layer_app::RasterTileLayerApp;
use crate::apps::organisms::main_window::map::vector_layer_app::VectorLayerApp;
use crate::domain::params::design_token_config::{HEADER_HEIGHT, LAYER_CONTROLLER_WIDTH};
use crate::domain::params::map_config::{
    DATA_PROJ_EPSG, MAP_CENTER_LATITUDE, MAP_CENTER_LONGITUDE, MAP_SCROLL_LINE_DELTA_PIXELS,
};
use crate::domain::traits::coordinate_transformer_trait::CoordinateTransformer;
use crate::domain::traits::load_raster_tile_json_trait::LoadRasterTileJsonTrait;
use crate::domain::traits::load_vector_json_trait::LoadVectorJsonTrait;
use crate::domain::traits::map_area_trait::MapAreaTrait;
use crate::domain::traits::vector_repository_trait::VectorRepositoryTrait;
use crate::domain::types::map_coordinate_type::{EpsgCoordinate, WebMercatorCoordinate};
use crate::domain::types::map_layer_type::{RasterTileLayer, VectorFeature, VectorLayer};
use crate::infrastructure::coordinate::proj_core_coordinate_transformer::ProjCoreCoordinateTransformer;
use crate::infrastructure::duckdb::vector_repository::DuckDbVectorRepository;
use crate::infrastructure::json::load_raster_tile_json::LoadRasterTileJson;
use crate::infrastructure::json::load_vector_json::LoadVectorJson;

#[derive(Default)]
//ドラッグ状態を規定する型
struct DragState {
    is_dragging: bool,
    last_position: Option<Point<Pixels>>,
    offset_x: f32,
    offset_y: f32,
}

pub struct MapChanged;

/// 地図の描画オフセットを保持する表示専用コンポーネント。
struct MapViewport {
    content: Entity<MapContent>,
    offset_x: f32,
    offset_y: f32,
}

impl MapViewport {
    /// 表示オフセットの初期状態を作成する。
    fn new(content: Entity<MapContent>) -> Self {
        Self {
            content,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }

    /// ドラッグ中の移動量を反映して表示位置を更新する。
    fn set_offset(&mut self, offset_x: f32, offset_y: f32, cx: &mut Context<Self>) {
        self.offset_x = offset_x;
        self.offset_y = offset_y;
        cx.notify();
    }

    /// オフセットをリセットしてドラッグ中の移動を消す。
    fn reset_offset(&mut self, cx: &mut Context<Self>) {
        self.set_offset(0.0, 0.0, cx);
    }
}

impl Render for MapViewport {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .absolute()
            .size_full()
            .top(px(self.offset_y))
            .left(px(self.offset_x))
            .child(self.content.clone())
    }
}

/// 実際の地図コンテンツを保持する表示専用コンポーネント。
pub struct MapContent {
    raster_layers: Vec<RasterTileLayer>,
    visible_tiles: Vec<MapTile>,
    vector_geometries: Vec<VectorGeometryCacheEntry>,
    show_vectors: bool,
}

impl MapContent {
    /// 地図描画に必要な初期状態を構築する。
    pub fn new(
        raster_layers: Vec<RasterTileLayer>,
        visible_tiles: Vec<MapTile>,
        vector_geometries: Vec<VectorGeometryCacheEntry>,
    ) -> Self {
        Self {
            raster_layers,
            visible_tiles,
            vector_geometries,
            show_vectors: true,
        }
    }

    /// ドラッグ中のベクター描画を切り替える。
    fn set_vector_visibility(&mut self, visible: bool, cx: &mut Context<Self>) {
        if self.show_vectors == visible {
            return;
        }
        self.show_vectors = visible;
        cx.notify();
    }

    /// 外部から受け取った状態を更新して、次回の描画で反映する。
    pub fn update_state(
        &mut self,
        raster_layers: Vec<RasterTileLayer>,
        visible_tiles: Vec<MapTile>,
        vector_geometries: Vec<VectorGeometryCacheEntry>,
        cx: &mut Context<Self>,
    ) {
        self.raster_layers = raster_layers;
        self.visible_tiles = visible_tiles;
        self.vector_geometries = vector_geometries;
        cx.notify();
    }
}

impl Render for MapContent {
    /// 保存済みの geometry とタイルをそのまま描画する。
    ///
    /// ここで再計算は行わず、MapApp が事前に計算した結果を再利用する。
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut content = div()
            .absolute()
            .size_full()
            .child(RasterTileLayerApp::render(
                self.visible_tiles.clone(),
                self.raster_layers.clone(),
            ));

        if self.show_vectors {
            content = content.child(VectorLayerApp::render(self.vector_geometries.clone()));
        }

        content
    }
}

pub struct MapApp {
    map_instance: MapInstance,
    raster_tile_layers: Vec<RasterTileLayer>,
    vector_layers: Vec<(VectorLayer, Vec<VectorFeature>)>,
    drag_state: DragState,
    content: Entity<MapContent>,
    viewport: Entity<MapViewport>,
    viewport_width: f32,
    viewport_height: f32,
    repository: DuckDbVectorRepository,
    geometry_cache_key: Option<GeometryCacheKey>,
    geometry_cache: Vec<(
        crate::domain::types::map_layer_type::VectorStyle,
        f32,
        Vec<crate::domain::traits::vector_layer_service_trait::ScreenGeometry>,
    )>,
}

impl EventEmitter<MapChanged> for MapApp {}

impl MapApp {
    /// 地図アプリの初期状態を生成する。
    ///
    /// 初期中心座標の計算、ベクターデータの読み込み、表示用の MapContent と
    /// MapViewport の構築を行い、画面の基本状態をセットアップする。
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let center = ProjCoreCoordinateTransformer
            .epsg_coordinate_to_web_mercator(EpsgCoordinate {
                x: MAP_CENTER_LONGITUDE,
                y: MAP_CENTER_LATITUDE,
                epsg: DATA_PROJ_EPSG,
            })
            .expect("failed to convert initial map center to Web Mercator");

        let map_instance = MapInstance::new(center);
        let viewport = window.viewport_size();
        let map_area =
            MapArea::get_map_area_size(f32::from(viewport.width), f32::from(viewport.height));
        let bbox = MapBbox::calculate(&map_instance, map_area.width, map_area.height);
        let repository = DuckDbVectorRepository::new().expect("failed to initialize DuckDB");
        let vector_layers: Vec<(VectorLayer, Vec<VectorFeature>)> = LoadVectorJson::load()
            .into_iter()
            .filter_map(
                |layer| match repository.load_features_in_bbox(&layer, bbox) {
                    Ok(features) => Some((layer, features)),
                    Err(err) => {
                        eprintln!("failed to load vector layer: {err:#}");
                        None
                    }
                },
            )
            .collect();

        let raster_layers = LoadRasterTileJson::load();
        let visible_tiles =
            MapRenderService::resolve_visible_tiles(&map_instance, map_area.width, map_area.height);
        let center_x = f64::from(LAYER_CONTROLLER_WIDTH) + f64::from(map_area.width) / 2.0;
        let center_y = f64::from(HEADER_HEIGHT) + f64::from(map_area.height) / 2.0;
        let geometry_cache = MapRenderService::resolve_vector_geometries(
            &vector_layers,
            &map_instance,
            (center_x, center_y),
        );
        let initial_cache_key = MapRenderService::build_geometry_cache_key(
            &map_instance,
            map_area.width,
            map_area.height,
        );
        let content = cx.new(|_| {
            MapContent::new(
                raster_layers.clone(),
                visible_tiles.clone(),
                geometry_cache.clone(),
            )
        });
        let viewport = cx.new(|_| MapViewport::new(content.clone()));

        Self {
            map_instance,
            raster_tile_layers: raster_layers,
            vector_layers,
            drag_state: DragState::default(),
            content,
            viewport,
            viewport_width: map_area.width,
            viewport_height: map_area.height,
            repository,
            geometry_cache_key: Some(initial_cache_key),
            geometry_cache,
        }
    }

    /// 地図の中心座標を更新し、関連するレイヤー再読込と再描画を実行する。
    pub fn set_center(&mut self, center: WebMercatorCoordinate, cx: &mut Context<Self>) {
        self.map_instance.set_center(center);
        self.vector_layers = MapStateService::reload_vector_layers(
            &self.repository,
            &self.map_instance,
            self.viewport_width,
            self.viewport_height,
        );
        self.on_change_event(cx);
    }

    /// 現在の地図状態への参照を返す。
    pub fn map(&self) -> &MapInstance {
        &self.map_instance
    }

    /// 指定したレイヤーの表示状態を切り替え、設定を保存して再描画を要求する。
    pub fn set_layer_visibility(&mut self, id: &str, visible: bool, cx: &mut Context<Self>) {
        if let Some(layer) = self
            .raster_tile_layers
            .iter_mut()
            .find(|layer| layer.id == id)
        {
            if layer.visible == visible {
                return;
            }
            layer.visible = visible;
            if let Err(err) = LoadRasterTileJson::save(&self.raster_tile_layers) {
                eprintln!("{err}");
            }
        } else if let Some((layer, _)) = self
            .vector_layers
            .iter_mut()
            .find(|(layer, _)| layer.id == id)
        {
            if layer.visible == visible {
                return;
            }
            layer.visible = visible;
            let layers = self
                .vector_layers
                .iter()
                .map(|(layer, _)| layer.clone())
                .collect::<Vec<_>>();
            if let Err(err) = LoadVectorJson::save(&layers) {
                eprintln!("{err}");
            }
        } else {
            return;
        }
        self.on_change_event(cx);
    }

    /// 指定したレイヤーの透明度を更新し、設定ファイルに反映して再描画する。
    pub fn set_layer_opacity(&mut self, id: &str, opacity: f32, cx: &mut Context<Self>) {
        let opacity = opacity.clamp(0.0, 1.0);
        if let Some(layer) = self
            .raster_tile_layers
            .iter_mut()
            .find(|layer| layer.id == id)
        {
            if (layer.opacity - opacity).abs() < f32::EPSILON {
                return;
            }
            layer.opacity = opacity;
            if let Err(err) = LoadRasterTileJson::save(&self.raster_tile_layers) {
                eprintln!("{err}");
            }
        } else if let Some((layer, _)) = self
            .vector_layers
            .iter_mut()
            .find(|(layer, _)| layer.id == id)
        {
            if (layer.opacity - opacity).abs() < f32::EPSILON {
                return;
            }
            layer.opacity = opacity;
            let layers = self
                .vector_layers
                .iter()
                .map(|(layer, _)| layer.clone())
                .collect::<Vec<_>>();
            if let Err(err) = LoadVectorJson::save(&layers) {
                eprintln!("{err}");
            }
        } else {
            return;
        }
        self.on_change_event(cx);
    }

    /// 地図状態が変わったことを MapContent と描画側へ通知する。
    ///
    /// viewport と zoom の key が変わったときだけ geometry を再計算し、
    /// それ以外では保存済みの描画結果をそのまま再利用する。
    fn on_change_event(&mut self, cx: &mut Context<Self>) {
        let raster_layers = self.raster_tile_layers.clone();
        let vector_layers = self.vector_layers.clone();
        let next_key = MapRenderService::build_geometry_cache_key(
            &self.map_instance,
            self.viewport_width,
            self.viewport_height,
        );

        let should_refresh_geometry = self
            .geometry_cache_key
            .as_ref()
            .map(|current| current != &next_key)
            .unwrap_or(true);

        if should_refresh_geometry {
            let center_x = f64::from(LAYER_CONTROLLER_WIDTH) + f64::from(self.viewport_width) / 2.0;
            let center_y = f64::from(HEADER_HEIGHT) + f64::from(self.viewport_height) / 2.0;
            self.geometry_cache = MapRenderService::resolve_vector_geometries(
                &vector_layers,
                &self.map_instance,
                (center_x, center_y),
            );
            self.geometry_cache_key = Some(next_key.clone());
        }

        let visible_tiles = MapRenderService::resolve_visible_tiles(
            &self.map_instance,
            self.viewport_width,
            self.viewport_height,
        );

        self.content.update(cx, |content, cx| {
            content.update_state(
                raster_layers,
                visible_tiles,
                self.geometry_cache.clone(),
                cx,
            );
        });
        cx.emit(MapChanged);
        cx.notify();
    }

    /// 現在の表示範囲に対応するベクターレイヤーを再読み込みする。
    ///
    /// 画面中心やビューサイズに基づいて bbox を計算し、表示領域内の地物だけを取得する。
    fn reload_vector_layers(&mut self) {
        self.vector_layers = MapStateService::reload_vector_layers(
            &self.repository,
            &self.map_instance,
            self.viewport_width,
            self.viewport_height,
        );
    }
}

impl Render for MapApp {
    /// 地図画面の描画とユーザー操作をまとめて処理する。
    ///
    /// ホイールズーム、マウスドラッグ、表示範囲の更新をここで制御し、
    /// `MapViewport` と `MapContent` を組み合わせて描画する。
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let viewport = window.viewport_size();
        let map_area =
            MapArea::get_map_area_size(f32::from(viewport.width), f32::from(viewport.height));
        self.viewport_width = map_area.width;
        self.viewport_height = map_area.height;
        let map_app = cx.entity();
        let map_app_for_scroll = map_app.clone();
        let map_app_for_mouse_down = map_app.clone();
        let map_app_for_mouse_move = map_app.clone();

        div()
            .absolute()
            .top(px(HEADER_HEIGHT))
            .bottom_0()
            .left(px(LAYER_CONTROLLER_WIDTH))
            .right_0()
            .on_scroll_wheel(move |event, _window, cx| {
                //ホイールイベント
                let delta = match event.delta {
                    ScrollDelta::Pixels(delta) => f32::from(delta.y),
                    ScrollDelta::Lines(delta) => delta.y * MAP_SCROLL_LINE_DELTA_PIXELS,
                };
                map_app_for_scroll.update(cx, |map_app, cx| {
                    MapStateService::zoom(&mut map_app.map_instance, delta);
                    map_app.reload_vector_layers();
                    map_app.on_change_event(cx);
                });
            })
            .on_mouse_down(MouseButton::Left, move |event, _window, cx| {
                map_app_for_mouse_down.update(cx, |map_app, _cx| {
                    // ドラッグ開始
                    map_app.drag_state.is_dragging = true;
                    // ドラッグ開始位置を記録
                    map_app.drag_state.last_position = Some(event.position);
                });
            })
            .on_mouse_move(move |event, _window, cx| {
                map_app_for_mouse_move.update(cx, |map_app, cx| {
                    // ドラッグ中の処理
                    if !map_app.drag_state.is_dragging {
                        return;
                    }
                    // ドラッグ中のマウス位置の変化量を計算
                    let Some(last_position) = map_app.drag_state.last_position else {
                        return;
                    };
                    // ドラッグ中のマウス位置の変化量を計算
                    let delta = event.position - last_position;
                    let content = map_app.content.clone();
                    content.update(cx, |content, cx| {
                        content.set_vector_visibility(false, cx);
                    });
                    // 確定済みの描画を再計算せず、表示中のレイヤー全体を移動する。
                    map_app.drag_state.offset_x += f32::from(delta.x);
                    map_app.drag_state.offset_y += f32::from(delta.y);
                    // ドラッグ中のマウス位置を更新
                    map_app.drag_state.last_position = Some(event.position);
                    let offset_x = map_app.drag_state.offset_x;
                    let offset_y = map_app.drag_state.offset_y;
                    let viewport = map_app.viewport.clone();
                    viewport.update(cx, |viewport, cx| {
                        viewport.set_offset(offset_x, offset_y, cx);
                    });
                });
            })
            .on_mouse_up(MouseButton::Left, move |_, _window, cx| {
                map_app.update(cx, |map_app, cx| {
                    let content = map_app.content.clone();
                    content.update(cx, |content, cx| {
                        content.set_vector_visibility(true, cx);
                    });
                    if map_app.drag_state.offset_x != 0.0 || map_app.drag_state.offset_y != 0.0 {
                        MapStateService::pan(
                            &mut map_app.map_instance,
                            map_app.drag_state.offset_x,
                            map_app.drag_state.offset_y,
                        );
                        map_app.drag_state.offset_x = 0.0;
                        map_app.drag_state.offset_y = 0.0;
                        let viewport = map_app.viewport.clone();
                        viewport.update(cx, |viewport, cx| viewport.reset_offset(cx));
                        map_app.reload_vector_layers();
                        map_app.on_change_event(cx);
                    }
                    map_app.drag_state.is_dragging = false;
                    map_app.drag_state.last_position = None;
                });
            })
            .child(self.viewport.clone())
    }
}
