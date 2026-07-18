use proj_core::Transform;

use crate::domain::traits::coordinate_transformer_trait::{
    CoordinateTransformError, CoordinateTransformer,
};
use crate::domain::types::map_coordinate::{WebMercatorCoordinate, Wgs84Coordinate};

pub struct ProjCoreCoordinateTransformer;

impl CoordinateTransformer for ProjCoreCoordinateTransformer {
    fn wgs84_to_web_mercator(
        &self,
        coordinate: Wgs84Coordinate,
    ) -> Result<WebMercatorCoordinate, CoordinateTransformError> {
        let transformer = Transform::new("EPSG:4326", "EPSG:3857")
            .map_err(|error| CoordinateTransformError(error.to_string()))?;
        let (x, y) = transformer
            .convert((coordinate.longitude_deg, coordinate.latitude_deg))
            .map_err(|error| CoordinateTransformError(error.to_string()))?;

        Ok(WebMercatorCoordinate { x, y })
    }

    fn web_mercator_to_wgs84(
        &self,
        coordinate: WebMercatorCoordinate,
    ) -> Result<Wgs84Coordinate, CoordinateTransformError> {
        let transformer = Transform::new("EPSG:3857", "EPSG:4326")
            .map_err(|error| CoordinateTransformError(error.to_string()))?;
        let (longitude_deg, latitude_deg) = transformer
            .convert((coordinate.x, coordinate.y))
            .map_err(|error| CoordinateTransformError(error.to_string()))?;

        Ok(Wgs84Coordinate {
            longitude_deg,
            latitude_deg,
        })
    }
}
