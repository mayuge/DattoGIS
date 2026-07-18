/// WGS84 の経緯度座標（表示・外部データ用）
#[derive(Debug, Clone, Copy)]
pub struct Wgs84Coordinate {
    pub longitude_deg: f64,
    pub latitude_deg: f64,
}

/// Web Mercator（EPSG:3857）のメートル座標（地図内部状態用）
#[derive(Debug, Clone, Copy, Default)]
pub struct WebMercatorCoordinate {
    pub x: f64,
    pub y: f64,
}
