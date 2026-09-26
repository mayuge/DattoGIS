use crate::domain::types::map_instance_type::MapInstance;

pub trait MapEventTrait {
    /// スクロール操作を地図のズーム変更として適用する。
    fn zoom_by_scroll(map: &mut MapInstance, scroll_delta_y: f32);
    /// ピクセル単位の移動量を地図中心の移動として適用する。
    fn pan_by_pixels(map: &mut MapInstance, delta_x: f32, delta_y: f32);
}
