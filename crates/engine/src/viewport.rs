use proj_core::Transform;

#[derive(Clone)]
pub struct ViewportTransform {
    center_pixel: (f64, f64),
    viewport_center: (f64, f64),
    origin: (f64, f64),
    resolution: f64,
    epsg_to_web_mercator: Transform,
}

impl ViewportTransform {
    pub fn new(
        center: (f64, f64),
        zoom: u32,
        viewport_center: (f64, f64),
        source_epsg: u32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if zoom > 30
            || !center.0.is_finite()
            || !center.1.is_finite()
            || !viewport_center.0.is_finite()
            || !viewport_center.1.is_finite()
        {
            return Err("Invalid viewport transform parameters".into());
        }

        let zoom = u8::try_from(zoom)?;
        let tms = ogc_tms::web_mercator_quad().extended_to(zoom)?;
        let matrix = tms.matrix(zoom)?;
        let origin = matrix.point_of_origin();
        let resolution = matrix.cell_size();
        let center_pixel = (
            (center.0 - origin.0) / resolution,
            (origin.1 - center.1) / resolution,
        );

        Ok(Self {
            center_pixel,
            viewport_center,
            origin,
            resolution,
            epsg_to_web_mercator: Transform::new(&format!("EPSG:{source_epsg}"), "EPSG:3857")?,
        })
    }

    pub fn project_coordinate(
        &self,
        coordinate: (f64, f64),
    ) -> Result<(f64, f64), Box<dyn std::error::Error>> {
        let (x, y) = self.epsg_to_web_mercator.convert(coordinate)?;
        let pixel_x = (x - self.origin.0) / self.resolution;
        let pixel_y = (self.origin.1 - y) / self.resolution;

        Ok((
            self.viewport_center.0 + pixel_x - self.center_pixel.0,
            self.viewport_center.1 + pixel_y - self.center_pixel.1,
        ))
    }
}

pub fn pan_web_mercator_center(
    center: (f64, f64),
    zoom: u32,
    delta: (f64, f64),
) -> Result<(f64, f64), Box<dyn std::error::Error>> {
    crate::map::move_xy_web_mercator_center(
        center,
        zoom,
        crate::types::MoveXY {
            x: delta.0,
            y: delta.1,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::{ViewportTransform, pan_web_mercator_center};

    #[test]
    fn viewport_transform_projects_center_to_viewport_center() {
        let transform = ViewportTransform::new((0.0, 0.0), 3, (320.0, 240.0), 3857).unwrap();

        assert_eq!(
            transform.project_coordinate((0.0, 0.0)).unwrap(),
            (320.0, 240.0)
        );
    }

    #[test]
    fn moving_uses_tile_matrix_resolution_and_bounds() {
        let (x, y) = pan_web_mercator_center((0.0, 0.0), 0, (1.0, 1.0)).unwrap();
        assert!(x < 0.0);
        assert!(y > 0.0);

        assert!(pan_web_mercator_center((0.0, 0.0), 0, (f64::NAN, 0.0)).is_err());
    }
}
