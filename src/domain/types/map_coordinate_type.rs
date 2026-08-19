/// 経緯度座標（表示・外部データ用）
#[derive(Debug, Clone, Copy)]
pub struct EpsgCoordinate {
    pub x: f64,
    pub y: f64,
    pub epsg: u32,
}

/// Web Mercator（EPSG:3857）のメートル座標（地図内部状態用）
#[derive(Debug, Clone, Copy, Default)]
pub struct WebMercatorCoordinate {
    pub x: f64,
    pub y: f64,
}
