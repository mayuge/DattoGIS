use crate::domain::types::map_coordinate_type::WebMercatorCoordinate;

pub trait WorldPixelTrait: Sized {
    /// Web Mercator 座標をワールドピクセルに変換する。
    fn convert_coordinate_to_pixel(coordinate: &WebMercatorCoordinate, zoom_level: u32) -> Self;
    /// ワールドピクセルを Web Mercator 座標に変換する。
    fn convert_pixel_to_coordinate(
        pixel_x: f64,
        pixel_y: f64,
        zoom_level: u32,
    ) -> WebMercatorCoordinate;
    /// 所属するタイル列番号を返す。
    fn tile_column(&self) -> u32;
    /// 所属するタイル行番号を返す。
    fn tile_row(&self) -> u32;
    /// タイル内の X オフセットを返す。
    fn pixel_offset_x(&self) -> f64;
    /// タイル内の Y オフセットを返す。
    fn pixel_offset_y(&self) -> f64;
}
