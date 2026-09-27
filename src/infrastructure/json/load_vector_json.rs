use crate::domain::traits::load_vector_json_trait::LoadVectorJsonTrait;
use crate::domain::types::map_layer_type::VectorLayer;

pub struct LoadVectorJson;

impl LoadVectorJsonTrait for LoadVectorJson {
    /// ベクターレイヤー設定を JSON ファイルから読み込む。
    fn load() -> Vec<VectorLayer> {
        // JSONファイルのパスを指定
        let config_path = std::path::PathBuf::from("assets/config/vector_config.json");
        // JSONファイルを読み込む
        let content = match std::fs::read_to_string(&config_path) {
            Ok(content) => content,
            Err(err) => {
                eprintln!(
                    "failed to read vector layer config {:?}: {}",
                    config_path, err
                );
                return vec![];
            }
        };
        // JSONをパースしてVec<VectorLayer>に変換
        let vector_layers: Vec<VectorLayer> =
            serde_json::from_str(&content).unwrap_or_else(|err| {
                eprintln!(
                    "failed to parse vector layer config {:?}: {}",
                    config_path, err
                );
                vec![]
            });
        vector_layers
    }

    fn save(layers: &[VectorLayer]) -> Result<(), String> {
        let config_path = std::path::PathBuf::from("assets/config/vector_config.json");
        let content = serde_json::to_string_pretty(layers)
            .map_err(|err| format!("failed to serialize vector layer config: {err}"))?;

        std::fs::write(&config_path, content).map_err(|err| {
            format!(
                "failed to write vector layer config {}: {err}",
                config_path.display()
            )
        })
    }
}
