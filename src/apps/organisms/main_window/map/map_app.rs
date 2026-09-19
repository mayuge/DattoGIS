use gpui::*;

use crate::apps::organisms::common::services::map::use_map_area::MapArea;
use crate::apps::organisms::common::services::map::use_map_bbox::MapBbox;
use crate::apps::organisms::common::services::map::use_map_event::MapEvent;
use crate::apps::organisms::common::services::map::use_map_instance::MapInstance;
use crate::apps::organisms::main_window::map::map_content_app::MapContent;
use crate::apps::organisms::main_window::map::map_viewport_app::MapViewport;
use crate::domain::params::design_token_config::{HEADER_HEIGHT, LAYER_CONTROLLER_WIDTH};
use crate::domain::params::map_config::{
    DATA_PROJ_EPSG, MAP_CENTER_LATITUDE, MAP_CENTER_LONGITUDE, MAP_SCROLL_LINE_DELTA_PIXELS,
};
use crate::domain::traits::coordinate_transformer_trait::CoordinateTransformer;
use crate::domain::traits::load_raster_tile_json_trait::LoadRasterTileJsonTrait;
use crate::domain::traits::load_vector_json_trait::LoadVectorJsonTrait;
use crate::domain::traits::map_area_trait::MapAreaTrait;
use crate::domain::traits::map_event_trait::MapEventTrait;
use crate::domain::traits::vector_repository_trait::VectorRepositoryTrait;
use crate::domain::types::map_coordinate_type::{EpsgCoordinate, WebMercatorCoordinate};
use crate::domain::types::map_layer_type::{RasterTileLayer, VectorFeature, VectorLayer};
use crate::infrastructure::coordinate::proj_core_coordinate_transformer::ProjCoreCoordinateTransformer;
use crate::infrastructure::duckdb::vector_repository::DuckDbVectorRepository;
use crate::infrastructure::json::load_raster_tile_json::LoadRasterTileJson;
use crate::infrastructure::json::load_vector_json::LoadVectorJson;

#[derive(Default)]
struct DragState {
    is_dragging: bool,
    last_position: Option<Point<Pixels>>,
    offset_x: f32,
    offset_y: f32,
}

pub struct MapChanged;

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
}

impl EventEmitter<MapChanged> for MapApp {}

impl MapApp {
    /// 初期位置、レイヤー、ドラッグ状態を持つ地図を生成する。
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
        let content = cx.new(|_| {
            MapContent::new(
                map_instance.clone(),
                raster_layers.clone(),
                vector_layers.clone(),
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
        }
    }

    /// 地図中心を更新し、購読者へ変更を通知する。
    pub fn set_center(&mut self, center: WebMercatorCoordinate, cx: &mut Context<Self>) {
        self.map_instance.set_center(center);
        self.reload_vector_layers();
        self.on_change_event(cx);
    }

    /// 現在の地図状態を読み取り専用で返す。
    pub fn map(&self) -> &MapInstance {
        &self.map_instance
    }

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

    /// 地図の状態変更を通知して再描画を要求する。
    fn on_change_event(&mut self, cx: &mut Context<Self>) {
        let map = self.map_instance.clone();
        let raster_layers = self.raster_tile_layers.clone();
        let vector_layers = self.vector_layers.clone();
        self.content.update(cx, |content, cx| {
            content.update_state(map, raster_layers, vector_layers, cx);
        });
        cx.emit(MapChanged);
        cx.notify();
    }

    fn reload_vector_layers(&mut self) {
        let bbox = if self.viewport_width > 0.0 && self.viewport_height > 0.0 {
            Some(MapBbox::calculate(
                &self.map_instance,
                self.viewport_width,
                self.viewport_height,
            ))
        } else {
            None
        };
        self.vector_layers = LoadVectorJson::load()
            .into_iter()
            .filter_map(|layer| {
                let result = match bbox {
                    Some(bbox) => self.repository.load_features_in_bbox(&layer, bbox),
                    None => self.repository.load_features(&layer),
                };
                match result {
                    Ok(features) => Some((layer, features)),
                    Err(err) => {
                        eprintln!("failed to reload vector layer: {err:#}");
                        None
                    }
                }
            })
            .collect();
    }
}

impl Render for MapApp {
    /// 地図タイルとスクロール・ドラッグ操作を描画する。
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
                    MapEvent::zoom_by_scroll(&mut map_app.map_instance, delta);
                    map_app.reload_vector_layers();
                    map_app.on_change_event(cx);
                });
            })
            .on_mouse_down(MouseButton::Left, move |event, _window, cx| {
                map_app_for_mouse_down.update(cx, |map_app, _| {
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
                    if map_app.drag_state.offset_x != 0.0 || map_app.drag_state.offset_y != 0.0 {
                        MapEvent::pan_by_pixels(
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
