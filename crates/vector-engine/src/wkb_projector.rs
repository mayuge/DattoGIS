use rayon::prelude::*;

use engine::ViewportTransform;

const PARALLEL_MIN_FEATURES: usize = 256;

#[derive(Clone, Debug, PartialEq)]
pub enum ScreenGeometry {
    Point((f32, f32)),
    LineString(Vec<(f32, f32)>),
    Polygon(Vec<Vec<(f32, f32)>>),
}

pub trait VectorGeometryProjector: Send + Sync {
    fn project_wkb_features(
        &self,
        features: &[&[u8]],
        transform: &ViewportTransform,
    ) -> Vec<Result<Vec<ScreenGeometry>, String>>;
}

#[derive(Default)]
pub struct WkbVectorGeometryProjector;

impl VectorGeometryProjector for WkbVectorGeometryProjector {
    fn project_wkb_features(
        &self,
        features: &[&[u8]],
        transform: &ViewportTransform,
    ) -> Vec<Result<Vec<ScreenGeometry>, String>> {
        if features.len() >= PARALLEL_MIN_FEATURES {
            features
                .par_iter()
                .map(|wkb| project_wkb(wkb, transform))
                .collect()
        } else {
            features
                .iter()
                .map(|wkb| project_wkb(wkb, transform))
                .collect()
        }
    }
}

enum Geometry {
    Point((f64, f64)),
    LineString(Vec<(f64, f64)>),
    Polygon(Vec<Vec<(f64, f64)>>),
    MultiPolygon(Vec<Vec<Vec<(f64, f64)>>>),
}

fn project_wkb(wkb: &[u8], transform: &ViewportTransform) -> Result<Vec<ScreenGeometry>, String> {
    let geometry = read_wkb(wkb)?;
    match geometry {
        Geometry::Point(position) => Ok(vec![ScreenGeometry::Point(project_position(
            position, transform,
        )?)]),
        Geometry::LineString(positions) => Ok(vec![ScreenGeometry::LineString(project_positions(
            positions, transform,
        )?)]),
        Geometry::Polygon(rings) => Ok(vec![ScreenGeometry::Polygon(project_rings(
            rings, transform,
        )?)]),
        Geometry::MultiPolygon(polygons) => polygons
            .into_iter()
            .map(|rings| project_rings(rings, transform).map(ScreenGeometry::Polygon))
            .collect(),
    }
}

fn read_wkb(wkb: &[u8]) -> Result<Geometry, String> {
    if wkb.len() < 5 || wkb[0] != 1 {
        return Err("expected little-endian WKB".into());
    }
    let mut type_offset = 1;
    let geometry_type = read_u32(wkb, &mut type_offset)?;
    let mut offset = 5;
    match geometry_type {
        1 => {
            let positions = read_positions(wkb, &mut offset, 1)?;
            Ok(Geometry::Point(positions[0]))
        }
        2 => {
            let count = read_u32(wkb, &mut offset)? as usize;
            Ok(Geometry::LineString(read_positions(
                wkb,
                &mut offset,
                count,
            )?))
        }
        3 => Ok(Geometry::Polygon(read_polygon_body(wkb, &mut offset)?)),
        6 => {
            let count = read_u32(wkb, &mut offset)? as usize;
            if count > wkb.len().saturating_sub(offset) / 9 {
                return Err("truncated MultiPolygon WKB".into());
            }
            let mut polygons = Vec::with_capacity(count);
            for _ in 0..count {
                if wkb.get(offset) != Some(&1) {
                    return Err("expected little-endian Polygon in MultiPolygon".into());
                }
                offset += 1;
                if read_u32(wkb, &mut offset)? != 3 {
                    return Err("expected Polygon in MultiPolygon".into());
                }
                polygons.push(read_polygon_body(wkb, &mut offset)?);
            }
            Ok(Geometry::MultiPolygon(polygons))
        }
        _ => Err(format!("unsupported WKB geometry type: {geometry_type}")),
    }
}

fn read_u32(wkb: &[u8], offset: &mut usize) -> Result<u32, String> {
    let end = offset
        .checked_add(4)
        .ok_or_else(|| "WKB offset overflow".to_string())?;
    let bytes = wkb
        .get(*offset..end)
        .ok_or_else(|| "truncated WKB integer".to_string())?;
    *offset = end;
    Ok(u32::from_le_bytes(
        bytes.try_into().expect("four-byte slice"),
    ))
}

fn read_f64(wkb: &[u8], offset: &mut usize) -> Result<f64, String> {
    let end = offset
        .checked_add(8)
        .ok_or_else(|| "WKB offset overflow".to_string())?;
    let bytes = wkb
        .get(*offset..end)
        .ok_or_else(|| "truncated WKB coordinate".to_string())?;
    *offset = end;
    Ok(f64::from_le_bytes(
        bytes.try_into().expect("eight-byte slice"),
    ))
}

fn read_positions(wkb: &[u8], offset: &mut usize, count: usize) -> Result<Vec<(f64, f64)>, String> {
    (0..count)
        .map(|_| Ok((read_f64(wkb, offset)?, read_f64(wkb, offset)?)))
        .collect()
}

fn read_polygon_body(wkb: &[u8], offset: &mut usize) -> Result<Vec<Vec<(f64, f64)>>, String> {
    let ring_count = read_u32(wkb, offset)? as usize;
    (0..ring_count)
        .map(|_| {
            let position_count = read_u32(wkb, offset)? as usize;
            read_positions(wkb, offset, position_count)
        })
        .collect()
}

fn project_position(
    position: (f64, f64),
    transform: &ViewportTransform,
) -> Result<(f32, f32), String> {
    transform
        .project_coordinate(position)
        .map(|(x, y)| (x as f32, y as f32))
        .map_err(|error| error.to_string())
}

fn project_positions(
    positions: Vec<(f64, f64)>,
    transform: &ViewportTransform,
) -> Result<Vec<(f32, f32)>, String> {
    positions
        .into_iter()
        .map(|position| project_position(position, transform))
        .collect()
}

fn project_rings(
    rings: Vec<Vec<(f64, f64)>>,
    transform: &ViewportTransform,
) -> Result<Vec<Vec<(f32, f32)>>, String> {
    rings
        .into_iter()
        .map(|ring| project_positions(ring, transform))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{ScreenGeometry, VectorGeometryProjector, WkbVectorGeometryProjector};
    use engine::ViewportTransform;

    fn point_wkb(x: f64, y: f64) -> Vec<u8> {
        let mut wkb = vec![1];
        wkb.extend(1_u32.to_le_bytes());
        wkb.extend(x.to_le_bytes());
        wkb.extend(y.to_le_bytes());
        wkb
    }

    #[test]
    fn projects_points_in_input_order_for_parallel_batches() {
        let transform = ViewportTransform::new((0.0, 0.0), 0, (100.0, 100.0), 3857).unwrap();
        let geometries = (0..300)
            .map(|x| point_wkb(f64::from(x) * 10_000.0, 0.0))
            .collect::<Vec<_>>();
        let feature_refs = geometries.iter().map(Vec::as_slice).collect::<Vec<_>>();
        let resolution = ogc_tms::web_mercator_quad().matrix(0).unwrap().cell_size();

        let projected = WkbVectorGeometryProjector.project_wkb_features(&feature_refs, &transform);

        assert_eq!(projected.len(), 300);
        for (x, result) in projected.into_iter().enumerate() {
            let geometries = result.unwrap();
            assert_eq!(geometries.len(), 1);
            let (screen_x, screen_y) = match &geometries[0] {
                ScreenGeometry::Point(point) => *point,
                _ => panic!("expected a projected point"),
            };
            let expected_x = 100.0 + x as f64 * 10_000.0 / resolution;
            assert!((f64::from(screen_x) - expected_x).abs() < 0.001);
            assert!((f64::from(screen_y) - 100.0).abs() < 0.001);
        }
    }

    #[test]
    fn reports_malformed_wkb_without_discarding_other_features() {
        let transform = ViewportTransform::new((0.0, 0.0), 0, (100.0, 100.0), 3857).unwrap();
        let valid = point_wkb(0.0, 0.0);
        let invalid = [0, 0, 0, 0, 0];

        let projected = WkbVectorGeometryProjector
            .project_wkb_features(&[valid.as_slice(), &invalid], &transform);

        assert!(projected[0].is_ok());
        assert!(projected[1].is_err());
    }

    #[test]
    fn rejects_truncated_multipolygon_counts_without_allocating_from_input() {
        let transform = ViewportTransform::new((0.0, 0.0), 0, (100.0, 100.0), 3857).unwrap();
        let mut invalid = vec![1];
        invalid.extend(6_u32.to_le_bytes());
        invalid.extend(u32::MAX.to_le_bytes());

        let projected = WkbVectorGeometryProjector.project_wkb_features(&[&invalid], &transform);

        assert!(projected[0].is_err());
    }
}
