use gpui::*;

use crate::apps::organisms::common::services::map::use_map_area::MapArea;
use crate::apps::organisms::common::services::map::use_map_event::MapEvent;
use crate::apps::organisms::common::services::map::use_map_instance::MapInstance;
use crate::apps::organisms::common::services::map::use_map_tile::MapTile;
use crate::apps::organisms::main_window::map::raster_tile_layer_app::RasterTileLayerApp;
use crate::apps::templates::main_window::MainWindow;
use crate::domain::params::design_token_config::{HEADER_HEIGHT, LAYER_CONTROLLER_WIDTH};
use crate::domain::params::map_config::MAP_SCROLL_LINE_DELTA_PIXELS;
use crate::domain::traits::map_area_trait::MapAreaTrait;
use crate::domain::traits::map_event_trait::MapEventTrait;
use crate::domain::traits::map_tile_trait::MapTileTrait;
use crate::domain::types::map_layer_type::RasterTileLayer;

#[derive(Default)]
struct DragState {
    is_dragging: bool,
    last_position: Option<Point<Pixels>>,
}

pub struct MapApp;

impl MapApp {
    pub fn render(
        window: &mut Window,
        cx: &mut Context<MainWindow>,
        map_instance: MapInstance,
        raster_tile_layers: Vec<RasterTileLayer>,
    ) -> impl IntoElement {
        let map_state = window.use_state(cx, |_, _| map_instance);
        let drag_state = window.use_state(cx, |_, _| DragState::default());

        let map = map_state.read(cx).clone();
        let viewport = window.viewport_size();
        let map_viewport =
            MapArea::get_map_area_size(f32::from(viewport.width), f32::from(viewport.height));

        let visible_tiles =
            MapTile::calculate_visible_tiles(&map, map_viewport.width, map_viewport.height);

        let drag_state_for_mouse_down = drag_state.clone();
        let drag_state_for_mouse_move = drag_state.clone();
        let drag_state_for_mouse_up = drag_state.clone();
        let map_state_for_scroll = map_state.clone();
        let map_state_for_drag = map_state.clone();
        let app_entity_for_scroll = cx.entity();
        let app_entity_for_drag = cx.entity();

        div()
            .absolute()
            .top(px(HEADER_HEIGHT))
            .bottom_0()
            .left(px(LAYER_CONTROLLER_WIDTH))
            .right_0()
            //ホイールイベント受け取りイベント
            .on_scroll_wheel(move |event, window, cx| {
                let delta = match event.delta {
                    ScrollDelta::Pixels(delta) => f32::from(delta.y),
                    ScrollDelta::Lines(delta) => delta.y * MAP_SCROLL_LINE_DELTA_PIXELS,
                };
                //ホイール時のズーム
                map_state_for_scroll.update(cx, |map, _| MapEvent::zoom_by_scroll(map, delta));

                let map_now = map_state_for_scroll.read(cx).clone();

                app_entity_for_scroll.update(cx, |app, cx| {
                    app.map = map_now;
                    cx.notify();
                });

                window.refresh();
            })
            //マウスクリック時のイベント
            .on_mouse_down(MouseButton::Left, move |event, window, cx| {
                drag_state_for_mouse_down.update(cx, |state, _| {
                    state.is_dragging = true;
                    state.last_position = Some(event.position);
                });
                window.refresh();
            })
            //ドラッグ中のイベント
            .on_mouse_move(move |event, window, cx| {
                let dragging = drag_state_for_mouse_move.read(cx).is_dragging;
                if !dragging {
                    return;
                }
                // マウスの前回位置を取得
                let Some(last_position) = drag_state_for_mouse_move.read(cx).last_position else {
                    return;
                };

                let delta = event.position - last_position;
                let delta_x = f32::from(delta.x);
                let delta_y = f32::from(delta.y);

                map_state_for_drag
                    .update(cx, |map, _| MapEvent::pan_by_pixels(map, delta_x, delta_y));
                let map_now = map_state_for_drag.read(cx).clone();

                app_entity_for_drag.update(cx, |app, cx| {
                    app.map = map_now;
                    cx.notify();
                });
                drag_state_for_mouse_move.update(cx, |state, _| {
                    state.last_position = Some(event.position);
                });

                window.refresh();
            })
            //ドラッグ終了時のイベント
            .on_mouse_up(MouseButton::Left, move |_, window, cx| {
                drag_state_for_mouse_up.update(cx, |state, _| {
                    state.is_dragging = false;
                    state.last_position = None;
                });
                window.refresh();
            })
            // レイヤー描画
            // ラスタータイル
            .child(RasterTileLayerApp::render(
                visible_tiles,
                raster_tile_layers,
            ))
    }
}
