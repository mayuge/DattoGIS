use crate::domain::map_coordinate::WebMercatorCoordinate;

pub trait WorldPixelTrait: Sized {
    fn convert_coordinate_to_pixel(coordinate: &WebMercatorCoordinate, zoom_level: u32) -> Self;
    fn tile_column(&self) -> u32;
    fn tile_row(&self) -> u32;
    fn pixel_offset_x(&self) -> f64;
    fn pixel_offset_y(&self) -> f64;
}
