use crate::domain::types::map_coordinate_type::WebMercatorCoordinate;

pub trait WorldPixelTrait: Sized {
    fn convert_coordinate_to_pixel(coordinate: &WebMercatorCoordinate, zoom_level: u32) -> Self;
    fn convert_pixel_to_coordinate(
        pixel_x: f64,
        pixel_y: f64,
        zoom_level: u32,
    ) -> WebMercatorCoordinate;
    fn tile_column(&self) -> u32;
    fn tile_row(&self) -> u32;
    fn pixel_offset_x(&self) -> f64;
    fn pixel_offset_y(&self) -> f64;
}
