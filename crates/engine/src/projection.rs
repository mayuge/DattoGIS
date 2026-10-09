use proj_core::Transform;

pub fn transform_coordinate(
    from_epsg: u32,
    to_epsg: u32,
    coordinate: (f64, f64),
) -> Result<(f64, f64), Box<dyn std::error::Error>> {
    let transform = Transform::new(&format!("EPSG:{from_epsg}"), &format!("EPSG:{to_epsg}"))?;
    Ok(transform.convert(coordinate)?)
}
