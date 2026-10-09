use ogc_tms::{BoundingBox, TileMatrixSet};
use proj_core::Transform;

use crate::types::{MapObject, MapXYZ, Screen, TileZXY};

pub fn validate_map_xyz(xyz: &MapXYZ) -> bool {
    (-90.0..=90.0).contains(&xyz.latitude)
        && (-180.0..=180.0).contains(&xyz.longitude)
        && xyz.zoom <= 30
}

pub fn validate_screen(screen: &Screen) -> bool {
    screen.width.is_finite()
        && screen.height.is_finite()
        && screen.width > 0.0
        && screen.height > 0.0
}

pub fn create_map_obj(
    xyz: MapXYZ,
    screen: Screen,
) -> Result<MapObject, Box<dyn std::error::Error>> {
    create_map_obj_with_overscan(xyz, screen, 0)
}

pub fn create_map_obj_with_overscan(
    xyz: MapXYZ,
    screen: Screen,
    overscan_tiles: u32,
) -> Result<MapObject, Box<dyn std::error::Error>> {
    if !validate_map_xyz(&xyz) {
        return Err("Invalid MapXYZ parameters".into());
    }
    if !validate_screen(&screen) {
        return Err("Invalid Screen parameters".into());
    }

    let zoom = xyz.zoom as u8;
    let tms: TileMatrixSet = ogc_tms::web_mercator_quad().extended_to(zoom)?;
    let forward = Transform::new("EPSG:4326", "EPSG:3857")?;
    let inverse = forward.inverse()?;

    let latitude = xyz
        .latitude
        .clamp(-ogc_tms::MERCATOR_LAT_LIMIT, ogc_tms::MERCATOR_LAT_LIMIT);
    let (center_x, center_y) = forward.convert((xyz.longitude, latitude))?;

    let resolution = tms.resolution(zoom)?;
    let matrix = tms.matrix(zoom)?;
    let tile_width = f64::from(matrix.tile_width());
    let tile_height = f64::from(matrix.tile_height());
    let overscan_x = f64::from(overscan_tiles) * tile_width;
    let overscan_y = f64::from(overscan_tiles) * tile_height;

    let half_width = (screen.width / 2.0 + overscan_x) * resolution;
    let half_height = (screen.height / 2.0 + overscan_y) * resolution;
    let left = center_x - half_width;
    let right = center_x + half_width;
    let bottom = center_y - half_height;
    let top = center_y + half_height;

    let viewport_left = center_x - screen.width * resolution / 2.0;
    let viewport_right = center_x + screen.width * resolution / 2.0;
    let viewport_bottom = center_y - screen.height * resolution / 2.0;
    let viewport_top = center_y + screen.height * resolution / 2.0;
    let projected_bbox = BoundingBox {
        left,
        bottom,
        right,
        top,
    };

    let first_tile = tms.xy_tile(left, top, zoom)?;
    let first_bounds = tms.xy_bounds(first_tile)?;
    let delta = [
        (first_bounds.left - left) / resolution - overscan_x,
        (top - first_bounds.top) / resolution - overscan_y,
    ];

    let tiles = tms
        .tiles(projected_bbox, zoom..=zoom)?
        .map(|tile| -> Result<TileZXY, std::num::TryFromIntError> {
            Ok(TileZXY {
                x: u32::try_from(tile.x)?,
                y: u32::try_from(tile.y)?,
                z: u32::try_from(tile.z)?,
                delta: [
                    delta[0] + (tile.x - first_tile.x) as f64 * tile_width,
                    delta[1] + (tile.y - first_tile.y) as f64 * tile_height,
                ],
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let (min_lon, min_lat) = inverse.convert((viewport_left, viewport_bottom))?;
    let (max_lon, max_lat) = inverse.convert((viewport_right, viewport_top))?;
    let bbox = [min_lon, min_lat, max_lon, max_lat];

    Ok(MapObject { bbox, tiles, delta })
}

#[cfg(test)]
mod tests {
    use super::{create_map_obj, create_map_obj_with_overscan};
    use crate::types::{MapXYZ, Screen};

    #[test]
    fn overscan_changes_tile_positions_but_not_viewport_bbox() {
        let xyz = MapXYZ {
            latitude: 0.0,
            longitude: 0.0,
            zoom: 2,
        };
        let screen = Screen {
            width: 256.0,
            height: 256.0,
        };
        let viewport = create_map_obj(xyz, screen).unwrap();
        let overscanned = create_map_obj_with_overscan(xyz, screen, 1).unwrap();

        assert_eq!(viewport.bbox, overscanned.bbox);
        assert!(overscanned.tiles.len() > viewport.tiles.len());
        assert!(overscanned.tiles.iter().any(|tile| tile.delta[0] < 0.0));
        assert!(overscanned.tiles.iter().any(|tile| tile.delta[1] < 0.0));
    }
}
