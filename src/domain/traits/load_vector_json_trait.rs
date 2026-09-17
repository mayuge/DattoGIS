use crate::domain::types::map_layer_type::VectorLayer;

pub trait LoadVectorJsonTrait {
    /// ベクターレイヤー設定を読み込む。
    fn load() -> Vec<VectorLayer>;
    fn save(layers: &[VectorLayer]) -> Result<(), String>;
}
