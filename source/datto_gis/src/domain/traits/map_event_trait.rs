use crate::services::map::use_map_instance::MapInstance;

pub trait MapEventTrait {
    fn zoom_by_scroll(map: &mut MapInstance, scroll_delta_y: f32);
    fn pan_by_pixels(map: &mut MapInstance, delta_x: f32, delta_y: f32);
    fn coordinate_text(map: &MapInstance) -> String;
}
