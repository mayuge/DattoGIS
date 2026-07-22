use crate::domain::types::map_coordinate::{EpsgCoordinate, WebMercatorCoordinate};

#[derive(Debug)]
pub struct CoordinateTransformError(pub String);

//座標変換のトレイト
pub trait CoordinateTransformer {
    //epsgCoordinateからwebメルカトルへ変換
    fn epsg_coordinate_to_web_mercator(
        &self,
        coordinate: EpsgCoordinate,
    ) -> Result<WebMercatorCoordinate, CoordinateTransformError>;
    //webメルカトルからepsgCoordinateへ変換
    fn web_mercator_to_epsg_coordinate(
        &self,
        coordinate: WebMercatorCoordinate,
        epsg: u32,
    ) -> Result<EpsgCoordinate, CoordinateTransformError>;
}
