use std::cell::RefCell;
use std::collections::HashSet;
use std::path::Path;

use anyhow::{Context, Result, anyhow};
use duckdb::{Connection, params};
use serde_json::Value;

use crate::domain::traits::vector_repository_trait::VectorRepositoryTrait;
use crate::domain::types::map_layer_type::{VectorFeature, VectorLayer};

pub struct DuckDbVectorRepository {
    connection: RefCell<Connection>,
    loaded_layers: RefCell<HashSet<String>>,
}

impl DuckDbVectorRepository {
    pub fn new() -> Result<Self> {
        let connection = Connection::open_in_memory().context("open DuckDB in-memory database")?;
        connection.execute_batch(
            "CREATE TABLE vector_features (
                layer_id VARCHAR NOT NULL,
                geometry_wkb BLOB NOT NULL,
                properties JSON NOT NULL,
                min_x DOUBLE NOT NULL,
                min_y DOUBLE NOT NULL,
                max_x DOUBLE NOT NULL,
                max_y DOUBLE NOT NULL
            )",
        )?;
        Ok(Self {
            connection: RefCell::new(connection),
            loaded_layers: RefCell::new(HashSet::new()),
        })
    }
}

impl VectorRepositoryTrait for DuckDbVectorRepository {
    fn load_features(&self, layer: &VectorLayer) -> Result<Vec<VectorFeature>> {
        self.load_features_in_bbox(
            layer,
            (
                f64::NEG_INFINITY,
                f64::NEG_INFINITY,
                f64::INFINITY,
                f64::INFINITY,
            ),
        )
    }

    fn load_features_in_bbox(
        &self,
        layer: &VectorLayer,
        bbox: (f64, f64, f64, f64),
    ) -> Result<Vec<VectorFeature>> {
        self.ensure_layer_loaded(layer)?;
        let connection = self.connection.borrow();

        let mut statement = connection.prepare(
            "SELECT geometry_wkb, properties::VARCHAR
             FROM vector_features
                         WHERE layer_id = ?
                             AND max_x >= ? AND min_x <= ?
                             AND max_y >= ? AND min_y <= ?",
        )?;
        let rows =
            statement.query_map(params![layer.id, bbox.0, bbox.2, bbox.1, bbox.3], |row| {
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

impl DuckDbVectorRepository {
    fn ensure_layer_loaded(&self, layer: &VectorLayer) -> Result<()> {
        // 同じレイヤーを繰り返し読み込まないよう、ロード済みIDを確認する。
        if self.loaded_layers.borrow().contains(&layer.id) {
            return Ok(());
        }
        // GeoJSONの各地物をWKB、検索用BBox、属性JSONに分けて保存する。
        let geojson = std::fs::read_to_string(Path::new(&layer.path))
            .with_context(|| format!("read GeoJSON: {}", layer.path))?;
        let document: Value = serde_json::from_str(&geojson).context("parse GeoJSON")?;
        let features = document
            .get("features")
            .and_then(Value::as_array)
            .ok_or_else(|| anyhow!("GeoJSON does not contain a features array"))?;
        let connection = self.connection.borrow();
        for feature in features {
            let geometry = feature
                .get("geometry")
                .ok_or_else(|| anyhow!("GeoJSON feature does not contain geometry"))?;
            let wkb = geometry_to_wkb(geometry)?;
            let (min_x, min_y, max_x, max_y) = geometry_bbox(geometry)?;
            let properties = feature
                .get("properties")
                .cloned()
                .unwrap_or_else(|| Value::Object(Default::default()))
                .to_string();
            connection.execute(
                "INSERT INTO vector_features VALUES (?, ?, ?::JSON, ?, ?, ?, ?)",
                params![layer.id, wkb, properties, min_x, min_y, max_x, max_y],
            )?;
        }
        self.loaded_layers.borrow_mut().insert(layer.id.clone());
        Ok(())
    }
}

fn geometry_to_wkb(geometry: &Value) -> Result<Vec<u8>> {
    // GeoJSONのgeometry typeに応じて、WKBの型番号と座標本体を組み立てる。
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
            write_polygon_body(&mut wkb, coordinates)?;
        }
        "MultiPolygon" => {
            let polygons = coordinates
                .as_array()
                .ok_or_else(|| anyhow!("MultiPolygon coordinates are invalid"))?;
            wkb.extend(6u32.to_le_bytes());
            wkb.extend((polygons.len() as u32).to_le_bytes());
            for polygon in polygons {
                // WKBのMultiPolygon内部要素は、Polygonの完全なWKBとして格納する。
                wkb.push(1);
                wkb.extend(3u32.to_le_bytes());
                write_polygon_body(&mut wkb, polygon)?;
            }
        }
        _ => return Err(anyhow!("unsupported GeoJSON geometry: {geometry_type}")),
    }
    Ok(wkb)
}

fn write_polygon_body(wkb: &mut Vec<u8>, coordinates: &Value) -> Result<()> {
    // リング数と各リングの座標を順に書く。先頭リングが外周、後続が穴になる。
    let rings = coordinates
        .as_array()
        .ok_or_else(|| anyhow!("Polygon coordinates are invalid"))?;
    wkb.extend((rings.len() as u32).to_le_bytes());
    for ring in rings {
        write_positions(wkb, ring)?;
    }
    Ok(())
}

fn write_positions(wkb: &mut Vec<u8>, positions: &Value) -> Result<()> {
    // LineStringまたはリングの頂点数を記録してから座標を追加する。
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
    // GeoJSONの各頂点からX/Yを取り出し、WKBのlittle-endian形式で書き込む。
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

fn geometry_bbox(geometry: &Value) -> Result<(f64, f64, f64, f64)> {
    // PolygonやMultiPolygonの入れ子も再帰走査し、全頂点を含むBBoxを求める。
    let mut positions = Vec::new();
    collect_positions(
        geometry
            .get("coordinates")
            .ok_or_else(|| anyhow!("GeoJSON geometry coordinates are missing"))?,
        &mut positions,
    )?;
    let first = positions
        .first()
        .ok_or_else(|| anyhow!("GeoJSON geometry has no coordinates"))?;
    let mut bbox = (first.0, first.1, first.0, first.1);
    for (x, y) in positions.into_iter().skip(1) {
        bbox.0 = bbox.0.min(x);
        bbox.1 = bbox.1.min(y);
        bbox.2 = bbox.2.max(x);
        bbox.3 = bbox.3.max(y);
    }
    Ok(bbox)
}

fn collect_positions(value: &Value, positions: &mut Vec<(f64, f64)>) -> Result<()> {
    let Some(values) = value.as_array() else {
        return Err(anyhow!("GeoJSON coordinates are invalid"));
    };
    // 数値2つの配列は座標、それ以外は形状階層として再帰的にたどる。
    if values.len() >= 2 && values[0].is_number() && values[1].is_number() {
        positions.push((
            values[0]
                .as_f64()
                .ok_or_else(|| anyhow!("invalid x coordinate"))?,
            values[1]
                .as_f64()
                .ok_or_else(|| anyhow!("invalid y coordinate"))?,
        ));
        return Ok(());
    }
    for value in values {
        collect_positions(value, positions)?;
    }
    Ok(())
}
