use gpui::*;

use crate::apps::organisms::common::services::map::use_map_area::MapArea;
use crate::apps::organisms::common::services::map::use_map_event::MapEvent;
use crate::apps::organisms::common::services::map::use_map_instance::MapInstance;
use crate::apps::organisms::common::services::map::use_map_tile::MapTile;
use crate::apps::organisms::main_window::map::raster_tile_layer_app::RasterTileLayerApp;
use crate::domain::params::design_token_config::{HEADER_HEIGHT, LAYER_CONTROLLER_WIDTH};
use crate::domain::params::map_config::{
    DATA_PROJ_EPSG, MAP_CENTER_LATITUDE, MAP_CENTER_LONGITUDE, MAP_SCROLL_LINE_DELTA_PIXELS,
};
use crate::domain::traits::coordinate_transformer_trait::CoordinateTransformer;
use crate::domain::traits::load_raster_tile_config_trait::LoadRasterTileConfigTrait;
use crate::domain::traits::map_area_trait::MapAreaTrait;
use crate::domain::traits::map_event_trait::MapEventTrait;
use crate::domain::traits::map_tile_trait::MapTileTrait;
use crate::domain::types::map_coordinate_type::{EpsgCoordinate, WebMercatorCoordinate};
use crate::domain::types::map_layer_type::RasterTileLayer;
use crate::infrastructure::coordinate::proj_core_coordinate_transformer::ProjCoreCoordinateTransformer;
use crate::infrastructure::json::load_raster_tile_config::LoadRasterTileConfig;

#[derive(Default)]
struct DragState {
    is_dragging: bool,
    last_position: Option<Point<Pixels>>,
}

pub struct MapChanged;

pub struct MapApp {
    map: MapInstance,
    raster_tile_layers: Vec<RasterTileLayer>,
    drag_state: DragState,
}

impl EventEmitter<MapChanged> for MapApp {}

impl MapApp {
    /// 初期位置、レイヤー、ドラッグ状態を持つ地図を生成する。
    pub fn new(_: &mut Context<Self>) -> Self {
        let center = ProjCoreCoordinateTransformer
            .epsg_coordinate_to_web_mercator(EpsgCoordinate {
                x: MAP_CENTER_LONGITUDE,
                y: MAP_CENTER_LATITUDE,
                epsg: DATA_PROJ_EPSG,
            })
            .expect("failed to convert initial map center to Web Mercator");

        Self {
            map: MapInstance::new(center),
            raster_tile_layers: LoadRasterTileConfig::load(),
            drag_state: DragState::default(),
        }
    }

    /// 地図中心を更新し、購読者へ変更を通知する。
    pub fn set_center(&mut self, center: WebMercatorCoordinate, cx: &mut Context<Self>) {
        self.map.center = center;
        self.changed(cx);
    }

    /// 現在の地図状態を読み取り専用で返す。
    pub fn map(&self) -> &MapInstance {
        &self.map
    }

    /// 地図の状態変更を通知して再描画を要求する。
    fn changed(&mut self, cx: &mut Context<Self>) {
        cx.emit(MapChanged);
        cx.notify();
    }
}

impl Render for MapApp {
    /// 地図タイルとスクロール・ドラッグ操作を描画する。
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let viewport = window.viewport_size();
        let map_viewport =
            MapArea::get_map_area_size(f32::from(viewport.width), f32::from(viewport.height));
        let visible_tiles =
            MapTile::calculate_visible_tiles(&self.map, map_viewport.width, map_viewport.height);
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
            .on_scroll_wheel(move |event, window, cx| {
                let delta = match event.delta {
                    ScrollDelta::Pixels(delta) => f32::from(delta.y),
                    ScrollDelta::Lines(delta) => delta.y * MAP_SCROLL_LINE_DELTA_PIXELS,
                };
                map_app_for_scroll.update(cx, |map_app, cx| {
                    MapEvent::zoom_by_scroll(&mut map_app.map, delta);
                    map_app.changed(cx);
                });
                window.refresh();
            })
            .on_mouse_down(MouseButton::Left, move |event, window, cx| {
                map_app_for_mouse_down.update(cx, |map_app, _| {
                    map_app.drag_state.is_dragging = true;
                    map_app.drag_state.last_position = Some(event.position);
                });
                window.refresh();
            })
            .on_mouse_move(move |event, window, cx| {
                map_app_for_mouse_move.update(cx, |map_app, cx| {
                    if !map_app.drag_state.is_dragging {
                        return;
                    }
                    let Some(last_position) = map_app.drag_state.last_position else {
                        return;
                    };
                    let delta = event.position - last_position;
                    MapEvent::pan_by_pixels(
                        &mut map_app.map,
                        f32::from(delta.x),
                        f32::from(delta.y),
                    );
                    map_app.drag_state.last_position = Some(event.position);
                    map_app.changed(cx);
                });
                window.refresh();
            })
            .on_mouse_up(MouseButton::Left, move |_, window, cx| {
                map_app.update(cx, |map_app, _| {
                    map_app.drag_state.is_dragging = false;
                    map_app.drag_state.last_position = None;
                });
                window.refresh();
            })
            .child(RasterTileLayerApp::render(
                visible_tiles,
                self.raster_tile_layers.clone(),
            ))
    }
}
