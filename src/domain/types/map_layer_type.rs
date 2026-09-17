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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VectorLayer {
    pub id: String,
    pub name: String,
    pub path: String,
    pub z_index: u8,
    pub opacity: f32,
    pub visible: bool,
    pub attribution: Option<String>,
    pub style: VectorStyle,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VectorStyle {
    pub fill_color: String,
    pub stroke_color: String,
    pub stroke_width: f32,
}
