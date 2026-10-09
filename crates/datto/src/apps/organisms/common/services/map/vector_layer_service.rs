use crate::domain::traits::vector_layer_service_trait::{ScreenGeometry, VectorLayerServiceTrait};
use crate::domain::types::map_coordinate_type::WebMercatorCoordinate;
use crate::domain::types::map_layer_type::VectorFeature;

pub struct VectorLayerService;

impl VectorLayerServiceTrait for VectorLayerService {
    fn screen_geometries(
        &self,
        features: &[VectorFeature],
        epsg: u32,
        center: WebMercatorCoordinate,
        zoom_level: u32,
        viewport_center: (f64, f64),
    ) -> Vec<ScreenGeometry> {
        let transform = match engine::ViewportTransform::new(
            (center.x, center.y),
            zoom_level,
            viewport_center,
            epsg,
        ) {
            Ok(transform) => transform,
            Err(error) => {
                eprintln!("failed to create vector viewport transform: {error}");
                return Vec::new();
            }
        };

        features
            .iter()
            .filter_map(|feature| read_wkb(&feature.geometry_wkb))
            .flat_map(|geometry| match geometry {
                Geometry::Point(position) => project_geometry(vec![position], &transform)
                    .and_then(|positions| positions.first().copied())
                    .map(ScreenGeometry::Point)
                    .into_iter()
                    .collect::<Vec<_>>(),
                Geometry::LineString(positions) => project_geometry(positions, &transform)
                    .map(ScreenGeometry::LineString)
                    .into_iter()
                    .collect(),
                Geometry::Polygon(rings) => project_polygon(rings, &transform)
                    .map(ScreenGeometry::Polygon)
                    .into_iter()
                    .collect(),
                Geometry::MultiPolygon(polygons) => polygons
                    .into_iter()
                    .filter_map(|rings| project_polygon(rings, &transform))
                    .map(ScreenGeometry::Polygon)
                    .collect(),
            })
            .collect()
    }
}

enum Geometry {
    Point((f64, f64)),
    LineString(Vec<(f64, f64)>),
    Polygon(Vec<Vec<(f64, f64)>>),
    MultiPolygon(Vec<Vec<Vec<(f64, f64)>>>),
}

fn read_wkb(wkb: &[u8]) -> Option<Geometry> {
    // 現在はlittle-endian WKBを読み、Point/LineString/Polygon/MultiPolygonを識別する。
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
            let mut offset = 5;
            Some(Geometry::LineString(read_positions(wkb, &mut offset)?))
        }
        3 => {
            let mut offset = 5;
            Some(Geometry::Polygon(read_polygon_body(wkb, &mut offset)?))
        }
        6 => {
            let mut offset = 5;
            let count = read_u32(wkb, &mut offset)? as usize;
            let mut polygons = Vec::with_capacity(count);
            for _ in 0..count {
                if *wkb.get(offset)? != 1 {
                    return None;
                }
                offset += 1;
                if read_u32(wkb, &mut offset)? != 3 {
                    return None;
                }
                polygons.push(read_polygon_body(wkb, &mut offset)?);
            }
            Some(Geometry::MultiPolygon(polygons))
        }
        _ => None,
    }
}

fn read_u32(wkb: &[u8], offset: &mut usize) -> Option<u32> {
    let value = u32::from_le_bytes(wkb.get(*offset..*offset + 4)?.try_into().ok()?);
    *offset += 4;
    Some(value)
}

fn read_positions(wkb: &[u8], offset: &mut usize) -> Option<Vec<(f64, f64)>> {
    // 頂点数分のX/Y値を読み、読み取り位置を次のWKB要素へ進める。
    let count = read_u32(wkb, offset)? as usize;
    (0..count)
        .map(|_| {
            let x = f64::from_le_bytes(wkb.get(*offset..*offset + 8)?.try_into().ok()?);
            *offset += 8;
            let y = f64::from_le_bytes(wkb.get(*offset..*offset + 8)?.try_into().ok()?);
            *offset += 8;
            Some((x, y))
        })
        .collect()
}

fn read_polygon_body(wkb: &[u8], offset: &mut usize) -> Option<Vec<Vec<(f64, f64)>>> {
    // 外周と内側リングを分けたまま読み、後段で穴として描ける形を保つ。
    let ring_count = read_u32(wkb, offset)? as usize;
    (0..ring_count)
        .map(|_| read_positions(wkb, offset))
        .collect()
}

fn project_geometry(
    positions: Vec<(f64, f64)>,
    transform: &engine::ViewportTransform,
) -> Option<Vec<(f32, f32)>> {
    positions
        .into_iter()
        .map(
            |coordinate| match transform.project_coordinate(coordinate) {
                Ok(point) => Some((point.0 as f32, point.1 as f32)),
                Err(error) => {
                    eprintln!("failed to project vector coordinate: {error}");
                    None
                }
            },
        )
        .collect()
}

fn project_polygon(
    rings: Vec<Vec<(f64, f64)>>,
    transform: &engine::ViewportTransform,
) -> Option<Vec<Vec<(f32, f32)>>> {
    rings
        .into_iter()
        .map(|ring| project_geometry(ring, transform))
        .collect()
}
