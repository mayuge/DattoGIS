use crate::domain::types::map_coordinate::{WebMercatorCoordinate, Wgs84Coordinate};

#[derive(Debug)]
pub struct CoordinateTransformError(pub String);

//座標変換のトレイト
pub trait CoordinateTransformer {
    //wgs84からwebメルカトルへ変換
    fn wgs84_to_web_mercator(
        &self,
        coordinate: Wgs84Coordinate,
    ) -> Result<WebMercatorCoordinate, CoordinateTransformError>;
    //webメルカトルからwgs84へ変換
    fn web_mercator_to_wgs84(
        &self,
        coordinate: WebMercatorCoordinate,
    ) -> Result<Wgs84Coordinate, CoordinateTransformError>;
}
