use crate::domain::map_coordinate::{WebMercatorCoordinate, Wgs84Coordinate};

#[derive(Debug)]
pub struct CoordinateTransformError(pub String);

pub trait CoordinateTransformer {
    fn wgs84_to_web_mercator(
        &self,
        coordinate: Wgs84Coordinate,
    ) -> Result<WebMercatorCoordinate, CoordinateTransformError>;

    fn web_mercator_to_wgs84(
        &self,
        coordinate: WebMercatorCoordinate,
    ) -> Result<Wgs84Coordinate, CoordinateTransformError>;
}
