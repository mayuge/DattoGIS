/// 初期表示中心座標
pub const MAP_CENTER_LONGITUDE: f64 = 137.0;

/// 初期表示中心緯度
pub const MAP_CENTER_LATITUDE: f64 = 38.5;

///最小緯度、最大緯度
/// Web Mercatorのタイル座標は緯度を -85.05112878 から 85.05112878 の範囲に制限する必要があるため、これらの値を定数として定義している
/// この範囲を超えると、タイル座標の計算が正しく行われなくなり、地図表示に問題が生じる可能性がある
/// この値は、Web Mercator投影法の特性に基づいており、地図表示の安定性を確保するために使用される
pub const MAP_MIN_LATITUDE: f64 = -85.05112878;
pub const MAP_MAX_LATITUDE: f64 = 85.05112878;

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
