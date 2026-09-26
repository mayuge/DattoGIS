#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GeometryCacheKey {
    pub zoom_level: u32,
    pub center_x: i64,
    pub center_y: i64,
    pub viewport_width: i32,
    pub viewport_height: i32,
}
