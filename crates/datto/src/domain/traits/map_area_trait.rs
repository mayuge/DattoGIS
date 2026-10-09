pub trait MapAreaTrait: Sized {
    /// ウィンドウサイズから地図表示領域のサイズを求める。
    fn get_map_area_size(window_width: f32, window_height: f32) -> Self;
    /// 地図表示領域の中心座標を返す。
    fn get_map_area_center(self) -> (f32, f32);
}
