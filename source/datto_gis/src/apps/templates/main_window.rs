use gpui::*;
use std::path::PathBuf;

use crate::services::map::use_map_instance::MapInstance;
use crate::services::map::use_map_tile::MapTile;

pub struct MapApp;

impl MapApp {
    pub fn render(window: &mut Window) -> impl IntoElement {
        // 現在は仮の表示サイズ
        // 後で Window から取得するように変更
        let viewport_width = 1280.0;
        let viewport_height = 720.0;

        let map = MapInstance::default();

        let visible_tiles = MapTile::calculate_visible_tiles(&map, viewport_width, viewport_height);

        div()
            .relative()
            .size_full()
            // ラスタータイル
            .children(visible_tiles.into_iter().map(|tile| {
                img(PathBuf::from(format!(
                    "cache/{}/{}/{}.png",
                    tile.zoom_level, tile.tile_column, tile.tile_row,
                )))
                .absolute()
                .left(px(tile.draw_x))
                .top(px(tile.draw_y))
                .w(px(256.0))
                .h(px(256.0))
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
