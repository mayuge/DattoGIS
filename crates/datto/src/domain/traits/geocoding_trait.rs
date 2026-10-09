use anyhow::Result;

use crate::domain::types::geocoding_type::GeocodedCoordinate;

pub trait GeocodingTrait {
    fn search_address(&self, address: &str) -> Result<Option<GeocodedCoordinate>>;
}
