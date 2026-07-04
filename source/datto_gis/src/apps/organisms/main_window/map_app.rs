use gpui::*;

use crate::domain::map_config::{DEFAULT_RASTER_TILE_URL, RASTER_TILE_SIZE};
use crate::services::map::use_map_instance::MapInstance;
use crate::services::map::use_map_tile::MapTile;

use std::path::PathBuf;

pub struct MapApp;

impl MapApp {
    pub fn render(window: &mut Window) -> impl IntoElement {
        let map = MapInstance::default();

        let viewport = window.viewport_size();
        let viewport_width = f32::from(viewport.width);
        let viewport_height = f32::from(viewport.height);

        let visible_tiles = MapTile::calculate_visible_tiles(&map, viewport_width, viewport_height);

        div()
            .relative()
            .size_full()
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
