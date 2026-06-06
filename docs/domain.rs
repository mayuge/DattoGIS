//! GISアプリケーションのドメイン定義

/// アプリケーション名
pub const APP_NAME: &str = "DattoGIS";

/// バージョン
pub const APP_VERSION: &str = "0.0.0";

/// 言語コード（日本語）
pub const LANGUAGE_JA: &str = "ja";

/// タイムゾーン（日本標準時）
pub const TIMEZONE_JST: &str = "Asia/Tokyo";

/// 2D地図の回転角度（北を上）
pub const ROTATE_2D_ANGLE: f64 = 0.0;

/// 3D地図の回転角度
pub const ROTATE_3D_ANGLE: f64 = 90.0;

/// 初期表示中心座標（東京）
pub const MAP_CENTER_LONGITUDE: f64 = 139.6917;

/// 初期表示中心緯度（東京）
pub const MAP_CENTER_LATITUDE: f64 = 35.6895;

/// 初期ズームレベル
pub const MAP_ZOOM_LEVEL: f64 = 8.0;

/// 表示座標系（Web Mercator）
pub const DISPLAY_PROJ_EPSG: u32 = 3857;

/// データ座標系（WGS84）
pub const DATA_PROJ_EPSG: u32 = 4326;

/// デフォルトラスタタイル
pub const DEFAULT_RASTER_TILE_URL: &str =
    "https://cyberjapandata.gsi.go.jp/xyz/std/{z}/{x}/{y}.png";

/// デフォルトファイル形式
pub const DEFAULT_FILE_FORMAT: &str = "geojson";

/// カラー定義
pub const COLOR_BLACK: &str = "#111111";
pub const COLOR_WHITE: &str = "#ffffff";

pub const COLOR_PRIMARY: &str = "#e6e6e6";
pub const COLOR_SECONDARY: &str = "#b3b3b3";

pub const COLOR_SUCCESS: &str = "#52c41a";
pub const COLOR_WARNING: &str = "#fa5b01";
pub const COLOR_DANGER: &str = "#ff4d4f";

pub const COLOR_GRAY_10: &str = "#1a1a1a";
pub const COLOR_GRAY_20: &str = "#333333";
pub const COLOR_GRAY_30: &str = "#4d4d4d";
pub const COLOR_GRAY_40: &str = "#666666";
pub const COLOR_GRAY_50: &str = "#808080";
pub const COLOR_GRAY_60: &str = "#999999";
pub const COLOR_GRAY_70: &str = "#b3b3b3";
pub const COLOR_GRAY_80: &str = "#cccccc";
pub const COLOR_GRAY_90: &str = "#e6e6e6";