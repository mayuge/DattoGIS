pub fn bounding_box(
    coordinates: impl IntoIterator<Item = (f64, f64)>,
) -> Result<[f64; 4], Box<dyn std::error::Error>> {
    let mut coordinates = coordinates.into_iter();
    let (x, y) = coordinates.next().ok_or("No coordinates provided")?;
    if !x.is_finite() || !y.is_finite() {
        return Err("Coordinates must be finite".into());
    }

    let mut bbox = [x, y, x, y];
    for (x, y) in coordinates {
        if !x.is_finite() || !y.is_finite() {
            return Err("Coordinates must be finite".into());
        }
        bbox[0] = bbox[0].min(x);
        bbox[1] = bbox[1].min(y);
        bbox[2] = bbox[2].max(x);
        bbox[3] = bbox[3].max(y);
    }
    Ok(bbox)
}

#[cfg(test)]
mod tests {
    use super::bounding_box;

    #[test]
    fn calculates_coordinate_bounds() {
        assert_eq!(
            bounding_box([(3.0, 4.0), (-2.0, 8.0), (5.0, -1.0)]).unwrap(),
            [-2.0, -1.0, 5.0, 8.0]
        );
        assert!(bounding_box([]).is_err());
    }
}
