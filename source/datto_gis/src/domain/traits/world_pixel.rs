use crate::services::map::use_map_instance::Coordinate;

pub trait WorldPixelUseCase: Sized {
    fn from_coordinate(coordinate: &Coordinate, zoom_level: u32) -> Self;
    fn tile_column(&self) -> u32;
    fn tile_row(&self) -> u32;
    fn pixel_offset_x(&self) -> f64;
    fn pixel_offset_y(&self) -> f64;
}
