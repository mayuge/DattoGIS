use crate::domain::traits::coordinate_transformer_trait::{
    CoordinateTransformError, CoordinateTransformer,
};
use crate::domain::types::map_coordinate_type::{EpsgCoordinate, WebMercatorCoordinate};

pub struct ProjCoreCoordinateTransformer;

impl CoordinateTransformer for ProjCoreCoordinateTransformer {
    //座標系指定つき座標からウェブメルカトルへ変換
    /// 指定 EPSG の座標を Web Mercator 座標へ変換する。
    fn epsg_coordinate_to_web_mercator(
        &self,
        coordinate: EpsgCoordinate,
    ) -> Result<WebMercatorCoordinate, CoordinateTransformError> {
        let (x, y) =
            engine::transform_coordinate(coordinate.epsg, 3857, (coordinate.x, coordinate.y))
                .map_err(|error| CoordinateTransformError(error.to_string()))?;

        Ok(WebMercatorCoordinate { x, y })
    }
    //ウェブメルカトルから座標系指定つき座標へ変換
    /// Web Mercator 座標を指定 EPSG の座標へ変換する。
    fn web_mercator_to_epsg_coordinate(
        &self,
        coordinate: WebMercatorCoordinate,
        epsg: u32,
    ) -> Result<EpsgCoordinate, CoordinateTransformError> {
        let (x, y) = engine::transform_coordinate(3857, epsg, (coordinate.x, coordinate.y))
            .map_err(|error| CoordinateTransformError(error.to_string()))?;

        Ok(EpsgCoordinate { x, y, epsg })
    }
}
