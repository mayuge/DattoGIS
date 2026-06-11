/// 初期表示中心座標
pub const MAP_CENTER_LONGITUDE: f64 = 137;

/// 初期表示中心緯度
pub const MAP_CENTER_LATITUDE: f64 = 38.5;

/// 初期ズームレベル
pub const MAP_ZOOM_LEVEL: f64 = 5.0;

/// 表示座標系（Web Mercator）
pub const DISPLAY_PROJ_EPSG: u32 = 3857;

/// データ座標系（WGS84）
pub const DATA_PROJ_EPSG: u32 = 4326;

/// デフォルトラスタタイル
pub const DEFAULT_RASTER_TILE_URL: &str =
    "https://cyberjapandata.gsi.go.jp/xyz/std/{z}/{x}/{y}.png";

/// デフォルトベクターファイル形式
pub const DEFAULT_FILE_FORMAT: &str = "geojson";