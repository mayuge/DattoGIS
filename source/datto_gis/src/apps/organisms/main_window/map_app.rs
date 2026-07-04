use gpui::*;

use crate::apps::app::App as AppState;
use crate::domain::map_config::{
    DEFAULT_RASTER_TILE_URL, MAP_MAX_LATITUDE, MAP_MIN_LATITUDE, RASTER_TILE_SIZE,
};
use crate::services::map::use_map_instance::MapInstance;
use crate::services::map::use_map_tile::MapTile;

use std::path::PathBuf;

#[derive(Default)]
struct DragState {
    is_dragging: bool,
    last_position: Option<Point<Pixels>>,
}

pub struct MapApp;

impl MapApp {
    pub fn render(window: &mut Window, cx: &mut Context<AppState>) -> impl IntoElement {
        let map_state = window.use_state(cx, |_, _| MapInstance::default());
        let drag_state = window.use_state(cx, |_, _| DragState::default());

        let map = map_state.read(cx).clone();
        let viewport = window.viewport_size();
        let viewport_width = f32::from(viewport.width);
        let viewport_height = f32::from(viewport.height);

        let visible_tiles = MapTile::calculate_visible_tiles(&map, viewport_width, viewport_height);

        let drag_state_for_mouse_down = drag_state.clone();
        let drag_state_for_mouse_move = drag_state.clone();
        let drag_state_for_mouse_up = drag_state.clone();
        let map_state_for_handlers = map_state.clone();

        div()
            .relative()
            .size_full()
            .on_mouse_down(MouseButton::Left, move |event, window, cx| {
                drag_state_for_mouse_down.update(cx, |state, _| {
                    state.is_dragging = true;
                    state.last_position = Some(event.position);
                });
                window.refresh();
            })
            .on_mouse_move(move |event, window, cx| {
                let dragging = drag_state_for_mouse_move.read(cx).is_dragging;
                if !dragging {
                    return;
                }

                let Some(last_position) = drag_state_for_mouse_move.read(cx).last_position else {
                    return;
                };

                let delta = event.position - last_position;
                let delta_x = f32::from(delta.x) as f64;
                let delta_y = f32::from(delta.y) as f64;

                map_state_for_handlers.update(cx, |map, _| {
                    let zoom_level = map.zoom_level.round() as u32;
                    let world_size = RASTER_TILE_SIZE as f64 * (1u32 << zoom_level) as f64;
                    let pixels_per_degree_longitude = world_size / 360.0;
                    let pixels_per_degree_latitude = world_size / 180.0;

                    map.center.longitude = (map.center.longitude
                        - delta_x / pixels_per_degree_longitude)
                        .clamp(-180.0, 180.0);
                    map.center.latitude = (map.center.latitude
                        + delta_y / pixels_per_degree_latitude)
                        .clamp(MAP_MIN_LATITUDE, MAP_MAX_LATITUDE);
                });

                drag_state_for_mouse_move.update(cx, |state, _| {
                    state.last_position = Some(event.position);
                });

                window.refresh();
            })
            .on_mouse_up(MouseButton::Left, move |_, window, cx| {
                drag_state_for_mouse_up.update(cx, |state, _| {
                    state.is_dragging = false;
                    state.last_position = None;
                });
                window.refresh();
            })
            // ラスタータイル
            .children(visible_tiles.into_iter().map(|tile| {
                let url = tile.generate_tile_url(DEFAULT_RASTER_TILE_URL);

                img(SharedString::from(url))
                    .absolute()
                    .left(px(tile.draw_x))
                    .top(px(tile.draw_y))
                    .w(px(RASTER_TILE_SIZE as f32))
                    .h(px(RASTER_TILE_SIZE as f32))
            }))
            // クロスヘア
            .child(
                img(PathBuf::from("assets/map/crosshair.svg"))
                    .absolute()
                    .top_1_2()
                    .left_1_2()
                    .w(px(32.0))
                    .h(px(32.0))
                    .ml(px(-16.0))
                    .mt(px(-16.0)),
            )
    }
}
