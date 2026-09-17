use crate::apps::organisms::common::services::map::use_map_world_pixel::WorldPixel;
use crate::domain::traits::coordinate_transformer_trait::CoordinateTransformer;
use crate::domain::traits::vector_layer_service_trait::{ScreenGeometry, VectorLayerServiceTrait};
use crate::domain::traits::world_pixel_trait::WorldPixelTrait;
use crate::domain::types::map_coordinate_type::{EpsgCoordinate, WebMercatorCoordinate};
use crate::domain::types::map_layer_type::VectorFeature;
use crate::infrastructure::coordinate::proj_core_coordinate_transformer::ProjCoreCoordinateTransformer;

pub struct VectorLayerService;

impl VectorLayerServiceTrait for VectorLayerService {
    fn screen_geometries(
        &self,
        features: &[VectorFeature],
        center: WebMercatorCoordinate,
        zoom_level: u32,
        viewport_center: (f64, f64),
    ) -> Vec<ScreenGeometry> {
        let center_pixel = WorldPixel::convert_coordinate_to_pixel(&center, zoom_level);
        features
            .iter()
            .filter_map(|feature| read_wkb(&feature.geometry_wkb))
            .filter_map(|geometry| match geometry {
                Geometry::Point(position) => project_geometry(
                    vec![position],
                    center_pixel,
                    zoom_level,
                    viewport_center,
                )
                .and_then(|positions| positions.first().copied())
                .map(ScreenGeometry::Point),
                Geometry::LineString(positions) => project_geometry(
                    positions,
                    center_pixel,
                    zoom_level,
                    viewport_center,
                )
                .map(ScreenGeometry::LineString),
            })
            .collect()
    }
}

enum Geometry {
    Point((f64, f64)),
    LineString(Vec<(f64, f64)>),
}

fn read_wkb(wkb: &[u8]) -> Option<Geometry> {
    if wkb.len() < 5 || wkb[0] != 1 {
        return None;
    }
    let geometry_type = u32::from_le_bytes(wkb[1..5].try_into().ok()?);
    match geometry_type {
        1 if wkb.len() >= 21 => Some(Geometry::Point((
            f64::from_le_bytes(wkb[5..13].try_into().ok()?),
            f64::from_le_bytes(wkb[13..21].try_into().ok()?),
        ))),
        2 => {
            let count = u32::from_le_bytes(wkb.get(5..9)?.try_into().ok()?) as usize;
            if wkb.len() < 9 + count * 16 {
                return None;
            }
            let positions = (0..count)
                .map(|index| {
                    let offset = 9 + index * 16;
                    Some((
                        f64::from_le_bytes(wkb[offset..offset + 8].try_into().ok()?),
                        f64::from_le_bytes(wkb[offset + 8..offset + 16].try_into().ok()?),
                    ))
                })
                .collect::<Option<Vec<_>>>()?;
            Some(Geometry::LineString(positions))
        }
        _ => None,
    }
}

fn project_geometry(
    positions: Vec<(f64, f64)>,
    center_pixel: WorldPixel,
    zoom_level: u32,
    viewport_center: (f64, f64),
) -> Option<Vec<(f32, f32)>> {
    positions
        .into_iter()
        .map(|(longitude, latitude)| {
            let coordinate = ProjCoreCoordinateTransformer
                .epsg_coordinate_to_web_mercator(EpsgCoordinate {
                    x: longitude,
                    y: latitude,
                    epsg: 4326,
                })
                .ok()?;
            let pixel = WorldPixel::convert_coordinate_to_pixel(&coordinate, zoom_level);
            Some((
                (viewport_center.0 + pixel.pixel_x - center_pixel.pixel_x) as f32,
                (viewport_center.1 + pixel.pixel_y - center_pixel.pixel_y) as f32,
            ))
        })
        .collect()
}