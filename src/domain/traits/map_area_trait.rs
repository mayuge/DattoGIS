pub trait MapAreaTrait: Sized {
    fn get_map_area_size(window_width: f32, window_height: f32) -> Self;
    fn get_map_area_center(self) -> (f32, f32);
}
