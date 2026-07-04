use gpui::*;

use crate::domain::map_config::DEFAULT_RASTER_TILE_URL;
use crate::services::map::use_map_instance::MapInstance;
use crate::services::map::use_map_tile::MapTile;

use std::path::PathBuf;

pub struct MapApp;

impl MapApp {
    pub fn render(_window: &mut Window) -> impl IntoElement {
        let map = MapInstance::default();

        // 仮の表示サイズ
        let viewport_width = 1280.0;
        let viewport_height = 720.0;

        let visible_tiles = MapTile::calculate_visible_tiles(&map, viewport_width, viewport_height);

        // タイルURL生成（次にダウンロード処理へ渡す）
        for tile in &visible_tiles {
            let tile_url = tile.generate_tile_url(DEFAULT_RASTER_TILE_URL);
            println!("{tile_url}");
        }

        div()
            .relative()
            .size_full()
            .child(div().size_full().bg(rgb(0xffffff)))
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
