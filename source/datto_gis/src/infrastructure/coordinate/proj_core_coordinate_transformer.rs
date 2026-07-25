use proj_core::Transform;

use crate::domain::traits::coordinate_transformer_trait::{
    CoordinateTransformError, CoordinateTransformer,
};
use crate::domain::types::map_coordinate_type::{EpsgCoordinate, WebMercatorCoordinate};

pub struct ProjCoreCoordinateTransformer;

impl CoordinateTransformer for ProjCoreCoordinateTransformer {
    //座標系指定つき座標からウェブメルカトルへ変換
    fn epsg_coordinate_to_web_mercator(
        &self,
        coordinate: EpsgCoordinate,
    ) -> Result<WebMercatorCoordinate, CoordinateTransformError> {
        let transformer = Transform::new(&format!("EPSG:{}", coordinate.epsg), "EPSG:3857")
            .map_err(|error| CoordinateTransformError(error.to_string()))?;
        let (x, y) = transformer
            .convert((coordinate.x, coordinate.y))
            .map_err(|error| CoordinateTransformError(error.to_string()))?;

        Ok(WebMercatorCoordinate { x, y })
    }
    //ウェブメルカトルから座標系指定つき座標へ変換
    fn web_mercator_to_epsg_coordinate(
        &self,
        coordinate: WebMercatorCoordinate,
        epsg: u32,
    ) -> Result<EpsgCoordinate, CoordinateTransformError> {
        let transformer = Transform::new("EPSG:3857", &format!("EPSG:{}", epsg))
            .map_err(|error| CoordinateTransformError(error.to_string()))?;
        let (x, y) = transformer
            .convert((coordinate.x, coordinate.y))
            .map_err(|error| CoordinateTransformError(error.to_string()))?;

        Ok(EpsgCoordinate { x, y, epsg })
    }
}
