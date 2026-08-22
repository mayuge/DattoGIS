use crate::domain::types::map_layer_type::RasterTileLayer;

pub trait LoadRasterTileConfigTrait {
    /// ラスタータイルレイヤー設定を読み込む。
    fn load() -> Vec<RasterTileLayer>;
}
