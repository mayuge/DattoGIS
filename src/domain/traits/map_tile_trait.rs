use crate::apps::organisms::services::map::use_map_instance::MapInstance;

pub trait MapTileTrait: Sized {
    fn generate_tile_url(&self, tile_url: &str) -> String;
    fn calculate_visible_tiles(
        map: &MapInstance,
        viewport_width: f32,
        viewport_height: f32,
    ) -> Vec<Self>;
}
