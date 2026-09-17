use std::path::Path;

use anyhow::{Context, Result, anyhow};
use duckdb::{Connection, params};
use serde_json::Value;

use crate::domain::traits::vector_repository_trait::VectorRepositoryTrait;
use crate::domain::types::map_layer_type::{VectorFeature, VectorLayer};

pub struct DuckDbVectorRepository;

impl VectorRepositoryTrait for DuckDbVectorRepository {
    fn load_features(&self, layer: &VectorLayer) -> Result<Vec<VectorFeature>> {
        let connection = Connection::open_in_memory().context("open DuckDB in-memory database")?;
        connection.execute_batch(
            "CREATE TABLE vector_features (
                layer_id VARCHAR NOT NULL,
                geometry_wkb BLOB NOT NULL,
                properties JSON NOT NULL
            )",
        )?;

        let geojson = std::fs::read_to_string(Path::new(&layer.path))
            .with_context(|| format!("read GeoJSON: {}", layer.path))?;
        let document: Value = serde_json::from_str(&geojson).context("parse GeoJSON")?;
        let features = document
            .get("features")
            .and_then(Value::as_array)
            .ok_or_else(|| anyhow!("GeoJSON does not contain a features array"))?;

        for feature in features {
            let geometry = feature
                .get("geometry")
                .ok_or_else(|| anyhow!("GeoJSON feature does not contain geometry"))?;
            let wkb = geometry_to_wkb(geometry)?;
            let properties = feature
                .get("properties")
                .cloned()
                .unwrap_or_else(|| Value::Object(Default::default()))
                .to_string();
            connection.execute(
                "INSERT INTO vector_features VALUES (?, ?, ?::JSON)",
                params![layer.id, wkb, properties],
            )?;
        }

        let mut statement = connection.prepare(
            "SELECT geometry_wkb, properties::VARCHAR
             FROM vector_features
             WHERE layer_id = ?",
        )?;
        let rows = statement.query_map(params![layer.id], |row| {
            let properties: String = row.get(1)?;
            Ok(VectorFeature {
                geometry_wkb: row.get(0)?,
                properties: serde_json::from_str(&properties).unwrap_or(Value::Null),
            })
        })?;

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .context("read vector features from DuckDB")
    }
}

fn geometry_to_wkb(geometry: &Value) -> Result<Vec<u8>> {
    let geometry_type = geometry
        .get("type")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("GeoJSON geometry type is missing"))?;
    let coordinates = geometry
        .get("coordinates")
        .ok_or_else(|| anyhow!("GeoJSON geometry coordinates are missing"))?;

    let mut wkb = vec![1];
    match geometry_type {
        "Point" => {
            wkb.extend(1u32.to_le_bytes());
            write_position(&mut wkb, coordinates)?;
        }
        "LineString" => {
            wkb.extend(2u32.to_le_bytes());
            write_positions(&mut wkb, coordinates)?;
        }
        "Polygon" => {
            wkb.extend(3u32.to_le_bytes());
            let rings = coordinates
                .as_array()
                .ok_or_else(|| anyhow!("Polygon coordinates are invalid"))?;
            wkb.extend((rings.len() as u32).to_le_bytes());
            for ring in rings {
                write_positions(&mut wkb, ring)?;
            }
        }
        _ => return Err(anyhow!("unsupported GeoJSON geometry: {geometry_type}")),
    }
    Ok(wkb)
}

fn write_positions(wkb: &mut Vec<u8>, positions: &Value) -> Result<()> {
    let positions = positions
        .as_array()
        .ok_or_else(|| anyhow!("GeoJSON positions are invalid"))?;
    wkb.extend((positions.len() as u32).to_le_bytes());
    for position in positions {
        write_position(wkb, position)?;
    }
    Ok(())
}

fn write_position(wkb: &mut Vec<u8>, position: &Value) -> Result<()> {
    let position = position
        .as_array()
        .ok_or_else(|| anyhow!("GeoJSON position is invalid"))?;
    let x = position
        .first()
        .and_then(Value::as_f64)
        .ok_or_else(|| anyhow!("GeoJSON position x is invalid"))?;
    let y = position
        .get(1)
        .and_then(Value::as_f64)
        .ok_or_else(|| anyhow!("GeoJSON position y is invalid"))?;
    wkb.extend(x.to_le_bytes());
    wkb.extend(y.to_le_bytes());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{DuckDbVectorRepository, VectorRepositoryTrait};
    use crate::domain::types::map_layer_type::{VectorLayer, VectorStyle};

    #[test]
    fn loads_geojson_into_duckdb_features() {
        let layer = VectorLayer {
            id: "airport".to_string(),
            name: "空港".to_string(),
            path: "assets/geo/airport.geojson".to_string(),
            z_index: 4,
            opacity: 1.0,
            visible: true,
            attribution: None,
            style: VectorStyle {
                fill_color: "#f59e0b".to_string(),
                stroke_color: "#ffffff".to_string(),
                stroke_width: 1.0,
            },
        };

        let features = DuckDbVectorRepository.load_features(&layer).unwrap();
        assert_eq!(features.len(), 97);
        assert_eq!(features[0].geometry_wkb[0], 1);
        assert!(features[0].properties.get("C28_000").is_some());
    }
}
