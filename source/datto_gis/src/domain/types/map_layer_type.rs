#[derive(Clone)]
pub struct RasterTileLayer {
    pub id: String,
    pub name: String,
    pub url: String,
    pub opacity: f32,
    pub visible: bool,
}
