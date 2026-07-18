/// 初期表示中心経度
pub const MAP_CENTER_LONGITUDE: f64 = 137.0;

/// 初期表示中心緯度
pub const MAP_CENTER_LATITUDE: f64 = 38.5;

/// Web Mercatorで扱える最小緯度
pub const MAP_MIN_LATITUDE: f64 = -85.05112878;

/// Web Mercatorで扱える最大緯度
pub const MAP_MAX_LATITUDE: f64 = 85.05112878;

pub const WEB_MERCATOR_HALF_WORLD_WIDTH: f64 = 20_037_508.342_789_244;

/// 初期ズームレベル
pub const MAP_ZOOM_LEVEL: f64 = 5.0;

/// Web Mercator（表示座標系）
pub const DISPLAY_PROJ_EPSG: u32 = 3857;

/// WGS84（データ座標系）
pub const DATA_PROJ_EPSG: u32 = 4326;

/// ラスタータイル1枚のサイズ（ピクセル）
pub const RASTER_TILE_SIZE: f64 = 256.0;

/// 画面外まで余分に読み込むタイル枚数
pub const RASTER_TILE_OVERSCAN: i32 = 1;

/// デフォルトラスタータイルURL
pub const DEFAULT_RASTER_TILE_URL: &str =
    "https://cyberjapandata.gsi.go.jp/xyz/std/{z}/{x}/{y}.png";

/// デフォルトベクターファイル形式
pub const DEFAULT_FILE_FORMAT: &str = "geojson";
