use crate::domain::traits::load_raster_tile_config_trait::LoadRasterTileConfigTrait;
use crate::domain::types::map_layer_type::RasterTileLayer;

pub struct LoadRasterTileConfig;

impl LoadRasterTileConfigTrait for LoadRasterTileConfig {
    // JSONファイルからラスタタイルの設定を読み込み
    /// ラスタータイルレイヤー設定を JSON ファイルから読み込む。
    fn load() -> Vec<RasterTileLayer> {
        // JSONファイルのパスを指定
        let config_path = std::path::PathBuf::from("assets/config/raster_tile_config.json");
        // JSONファイルを読み込む
        let content = match std::fs::read_to_string(&config_path) {
            Ok(content) => content,
            Err(err) => {
                eprintln!(
                    "failed to read raster tile config {:?}: {}",
                    config_path, err
                );
                return vec![];
            }
        };
        // JSONをパースしてVec<RasterTileLayer>に変換
        let raster_tile_layers: Vec<RasterTileLayer> = serde_json::from_str(&content)
            .unwrap_or_else(|err| {
                eprintln!(
                    "failed to parse raster tile config {:?}: {}",
                    config_path, err
                );
                vec![]
            });
        raster_tile_layers
    }
}
