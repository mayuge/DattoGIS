use ogc_tms::BoundingBox;
use proj_core::Transform;

use crate::types::{MapObject, MapXYZ, Screen, TileZXY};

#[derive(Clone)]
pub struct MapView {
    center: (f64, f64),
    zoom: u32,
    web_mercator_to_wgs84: Transform,
}

impl MapView {
    pub fn new(center: (f64, f64), zoom: u32) -> Result<Self, Box<dyn std::error::Error>> {
        if zoom > 30 || !center.0.is_finite() || !center.1.is_finite() {
            return Err("Invalid map view center or zoom".into());
        }
        Ok(Self {
            center,
            zoom,
            web_mercator_to_wgs84: Transform::new("EPSG:3857", "EPSG:4326")?,
        })
    }

    pub fn center(&self) -> (f64, f64) {
        self.center
    }

    pub fn zoom(&self) -> u32 {
        self.zoom
    }

    pub fn set_center(&mut self, center: (f64, f64)) {
        self.center = center;
    }

    pub fn set_zoom(&mut self, zoom: u32) -> Result<(), Box<dyn std::error::Error>> {
        if zoom > 30 {
            return Err("Invalid map view zoom".into());
        }
        self.zoom = zoom;
        Ok(())
    }

    /// Move the map center by a screen-relative pixel delta.
    pub fn move_xy(
        &mut self,
        movement: crate::types::MoveXY,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.center = move_xy_web_mercator_center(self.center, self.zoom, movement)?;
        Ok(())
    }

    pub fn create_map_obj(
        &self,
        screen: Screen,
        overscan_tiles: u32,
    ) -> Result<MapObject, Box<dyn std::error::Error>> {
        create_map_obj_from_center(
            self.center,
            self.zoom,
            screen,
            overscan_tiles,
            &self.web_mercator_to_wgs84,
        )
    }
}

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

    let forward = Transform::new("EPSG:4326", "EPSG:3857")?;

    let latitude = xyz
        .latitude
        .clamp(-ogc_tms::MERCATOR_LAT_LIMIT, ogc_tms::MERCATOR_LAT_LIMIT);
    let center = forward.convert((xyz.longitude, latitude))?;
    let inverse = forward.inverse()?;

    create_map_obj_from_center(center, xyz.zoom, screen, overscan_tiles, &inverse)
}

fn create_map_obj_from_center(
    center: (f64, f64),
    zoom: u32,
    screen: Screen,
    overscan_tiles: u32,
    inverse: &Transform,
) -> Result<MapObject, Box<dyn std::error::Error>> {
    if !validate_screen(&screen) {
        return Err("Invalid Screen parameters".into());
    }
    if zoom > 30 || !center.0.is_finite() || !center.1.is_finite() {
        return Err("Invalid map center or zoom".into());
    }

    let zoom = zoom as u8;
    let tms = ogc_tms::web_mercator_quad().extended_to(zoom)?;
    let resolution = tms.resolution(zoom)?;
    let matrix = tms.matrix(zoom)?;
    let tile_width = f64::from(matrix.tile_width());
    let tile_height = f64::from(matrix.tile_height());
    let overscan_x = f64::from(overscan_tiles) * tile_width;
    let overscan_y = f64::from(overscan_tiles) * tile_height;

    let half_width = (screen.width / 2.0 + overscan_x) * resolution;
    let half_height = (screen.height / 2.0 + overscan_y) * resolution;
    let left = center.0 - half_width;
    let right = center.0 + half_width;
    let bottom = center.1 - half_height;
    let top = center.1 + half_height;

    let viewport_left = center.0 - screen.width * resolution / 2.0;
    let viewport_right = center.0 + screen.width * resolution / 2.0;
    let viewport_bottom = center.1 - screen.height * resolution / 2.0;
    let viewport_top = center.1 + screen.height * resolution / 2.0;
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

    let center_wgs84 = inverse.convert(center)?;
    let (min_lon, min_lat) = inverse.convert((viewport_left, viewport_bottom))?;
    let (max_lon, max_lat) = inverse.convert((viewport_right, viewport_top))?;
    let bbox = [min_lon, min_lat, max_lon, max_lat];

    Ok(MapObject {
        center: [center_wgs84.0, center_wgs84.1],
        bbox,
        tiles,
        delta,
    })
}

pub fn move_xy_web_mercator_center(
    center: (f64, f64),
    zoom: u32,
    movement: crate::types::MoveXY,
) -> Result<(f64, f64), Box<dyn std::error::Error>> {
    if zoom > 30
        || !center.0.is_finite()
        || !center.1.is_finite()
        || !movement.x.is_finite()
        || !movement.y.is_finite()
    {
        return Err("Invalid map center, zoom, or move delta".into());
    }

    let zoom = zoom as u8;
    let tms = ogc_tms::web_mercator_quad().extended_to(zoom)?;
    let matrix = tms.matrix(zoom)?;
    let (origin_x, origin_y) = matrix.point_of_origin();
    let span_x = f64::from(matrix.tile_width()) * matrix.cell_size();
    let span_y = f64::from(matrix.tile_height()) * matrix.cell_size();
    let max_x = origin_x + matrix.matrix_width() as f64 * span_x;
    let min_y = origin_y - matrix.matrix_height() as f64 * span_y;

    Ok((
        (center.0 - movement.x * matrix.cell_size()).clamp(origin_x, max_x),
        (center.1 + movement.y * matrix.cell_size()).clamp(min_y, origin_y),
    ))
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

    #[test]
    fn map_view_moves_around_its_center_and_reuses_its_projection() {
        let mut view = super::MapView::new((0.0, 0.0), 2).unwrap();
        let screen = Screen {
            width: 256.0,
            height: 256.0,
        };
        let before = view.create_map_obj(screen, 0).unwrap();

        view.move_xy(crate::types::MoveXY { x: 10.0, y: 5.0 })
            .unwrap();
        let after = view.create_map_obj(screen, 0).unwrap();

        assert!(after.center[0] < before.center[0]);
        assert!(after.center[1] > before.center[1]);
        assert!(after.bbox[0] < before.bbox[0]);
        assert_eq!(after.tiles[0].z, view.zoom());
    }
}
