use crate::domain::types::map_layer_type::RasterTileLayer;

pub trait LoadRasterTileConfigTrait {
    fn load() -> Vec<RasterTileLayer>;
}
