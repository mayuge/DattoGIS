use crate::domain::types::map_coordinate_type::{EpsgCoordinate, WebMercatorCoordinate};

#[derive(Debug)]
pub struct CoordinateTransformError(pub String);

//座標変換のトレイト
pub trait CoordinateTransformer {
    //epsgCoordinateからwebメルカトルへ変換
    /// 指定 EPSG の座標を Web Mercator 座標へ変換する。
    fn epsg_coordinate_to_web_mercator(
        &self,
        coordinate: EpsgCoordinate,
    ) -> Result<WebMercatorCoordinate, CoordinateTransformError>;
    //webメルカトルからepsgCoordinateへ変換
    /// Web Mercator 座標を指定 EPSG の座標へ変換する。
    fn web_mercator_to_epsg_coordinate(
        &self,
        coordinate: WebMercatorCoordinate,
        epsg: u32,
    ) -> Result<EpsgCoordinate, CoordinateTransformError>;
}
