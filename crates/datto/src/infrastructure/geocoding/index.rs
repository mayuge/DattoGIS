use anyhow::{Result, anyhow};
use serde::Deserialize;
use url::form_urlencoded;

use crate::domain::params::api_config::GEOCODING_API_URL;
use crate::domain::traits::geocoding_trait::GeocodingTrait;
use crate::domain::types::geocoding_type::GeocodedCoordinate;

#[derive(Debug, Deserialize)]
struct GeocodingResponse(Vec<GeocodingFeature>);

#[derive(Debug, Deserialize)]
struct GeocodingFeature {
    geometry: Geometry,
}

#[derive(Debug, Deserialize)]
struct Geometry {
    coordinates: Vec<f64>,
}

pub struct Geocoding;

impl GeocodingTrait for Geocoding {
    /// 住所をジオコーディング API で検索し、最初の座標を返す。
    fn search_address(&self, address: &str) -> Result<Option<GeocodedCoordinate>> {
        let encoded_address: String = form_urlencoded::byte_serialize(address.as_bytes()).collect();
        let response = reqwest::blocking::get(format!("{GEOCODING_API_URL}{encoded_address}"))?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "geocoding API returned status {}",
                response.status()
            ));
        }

        let features: GeocodingResponse = serde_json::from_slice(&response.bytes()?)?;
        let Some(feature) = features.0.first() else {
            return Ok(None);
        };

        let [longitude, latitude, ..] = feature.geometry.coordinates.as_slice() else {
            return Err(anyhow!("geocoding API returned invalid coordinates"));
        };

        Ok(Some(GeocodedCoordinate {
            longitude: *longitude,
            latitude: *latitude,
        }))
    }
}
