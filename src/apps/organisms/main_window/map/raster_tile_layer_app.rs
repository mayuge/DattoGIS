use crate::domain::params::map_config::RASTER_TILE_SIZE;
use crate::domain::traits::map_tile_trait::MapTileTrait;
use crate::domain::types::map_layer_type::RasterTileLayer;
use crate::services::map::use_map_tile::MapTile;
use gpui::*;

pub struct RasterTileLayerApp;

impl RasterTileLayerApp {
    pub fn render(
        //各タイルの{z}{x}{y}の数値のリストを保持
        visible_tiles: Vec<MapTile>,
        //レイヤーのリスト
        raster_tile_layers: Vec<RasterTileLayer>,
    ) -> impl IntoElement {
        let mut layers: Vec<_> = raster_tile_layers
            .into_iter()
            .filter(|layer| layer.visible)
            .collect();
        layers.sort_by_key(|layer| layer.z_index);

        div().children(layers.into_iter().flat_map(|layer| {
            let opacity = layer.opacity;
            let url = layer.url.clone();

            visible_tiles.iter().map(move |tile| {
                img(SharedString::from(tile.generate_tile_url(&url)))
                    .absolute()
                    .left(px(tile.draw_x))
                    .top(px(tile.draw_y))
                    .w(px(RASTER_TILE_SIZE as f32))
                    .h(px(RASTER_TILE_SIZE as f32))
                    .opacity(opacity)
            })
        }))
    }
}
