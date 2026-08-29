use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RasterTileLayer {
    pub id: String,
    pub name: String,
    pub url: String,
    pub z_index: u8,
    pub opacity: f32,
    pub visible: bool,
    pub attribution: Option<String>,
}
