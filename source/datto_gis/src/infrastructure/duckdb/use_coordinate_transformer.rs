use duckdb::Connection;

use crate::domain::traits::coordinate_transformer_trait::{
    CoordinateTransformError, CoordinateTransformer,
};
use crate::domain::types::map_coordinate::{EpsgCoordinate, WebMercatorCoordinate};

pub struct DuckDbCoordinateTransformer {
    conn: Connection,
}

impl DuckDbCoordinateTransformer {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }
}

impl CoordinateTransformer for DuckDbCoordinateTransformer {
    fn epsg_coordinate_to_web_mercator(
        &self,
        coordinate: EpsgCoordinate,
    ) -> Result<WebMercatorCoordinate, CoordinateTransformError> {
        let sql = format!(
            r#"
            SELECT
                ST_X(t),
                ST_Y(t)
            FROM (
                SELECT ST_Transform(
                    ST_Point(?, ?),
                    'EPSG:{}',
                    'EPSG:3857'
                ) AS t
            )
            "#,
            coordinate.epsg
        );

        let (x, y): (f64, f64) = self
            .conn
            .query_row(&sql, [coordinate.x, coordinate.y], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })
            .map_err(|e| CoordinateTransformError(e.to_string()))?;

        Ok(WebMercatorCoordinate { x, y })
    }

    fn web_mercator_to_epsg_coordinate(
        &self,
        coordinate: WebMercatorCoordinate,
        epsg: u32,
    ) -> Result<EpsgCoordinate, CoordinateTransformError> {
        let sql = format!(
            r#"
            SELECT
                ST_X(t),
                ST_Y(t)
            FROM (
                SELECT ST_Transform(
                    ST_Point(?, ?),
                    'EPSG:3857',
                    'EPSG:{}'
                ) AS t
            )
            "#,
            epsg
        );

        let (x, y): (f64, f64) = self
            .conn
            .query_row(&sql, [coordinate.x, coordinate.y], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })
            .map_err(|e| CoordinateTransformError(e.to_string()))?;

        Ok(EpsgCoordinate { x, y, epsg })
    }
}
