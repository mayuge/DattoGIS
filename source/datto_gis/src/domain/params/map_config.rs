/// 初期表示中心経度
pub const MAP_CENTER_LONGITUDE: f64 = 137.0;

/// 初期表示中心緯度
pub const MAP_CENTER_LATITUDE: f64 = 38.5;


///Annex D: Common TileMatrixSet definitions (Informative)
///BBOX LowerCorner: -20037508.3427892 -20037508.3427892 (lat/long: -85.0511287798,-180)
///BBOX UpperCorner: 20037508.3427892 20037508.3427892 (lat/long: 85.0511287798,180)

/// Web Mercatorで扱える最小緯度
pub const MAP_MIN_LATITUDE: f64 = -85.05112878;

/// Web Mercatorで扱える最大緯度
pub const MAP_MAX_LATITUDE: f64 = 85.05112878;


/// Web Mercator（EPSG:3857）における、原点から世界端までの距離（メートル）。
///
/// 地球の半周長 `π × 6_378_137m` から求められる値で、X・Y 座標の有効範囲は
/// `-WEB_MERCATOR_HALF_WORLD_WIDTH..=WEB_MERCATOR_HALF_WORLD_WIDTH` となる。
/// WGS 84 楕円体の長半径 6,378,137m
/// OGC は EPSG:3857 のタイル行列範囲を、X・Y ともに ±20,037,508.3427892 m と定義しています
/// https://docs.ogc.org/is/17-083r2/17-083r2.html
/// Annex D: Common TileMatrixSet definitions (Informative)
///This Annex includes some definitions for TileMatrixSets that are commonly used.
///D.1      Web Mercator Quad TileMatrixSet definition (http://www.opengis.net/def/tilematrixset/OGC/1.0/WebMercatorQuad).
///Table D.1 — Definition of the WebMercatorQuad TileMatrixSet
///CRS: http://www.opengis.net/def/crs/EPSG/0/3857, WGS 84 / Pseudo-Mercator
///BBOX LowerCorner: -20037508.3427892 -20037508.3427892 (lat/long: -85.0511287798,-180)
///BBOX UpperCorner: 20037508.3427892 20037508.3427892 (lat/long: 85.0511287798,180)
///WellKnownScaleSet: http://www.opengis.net/def/wkss/OGC/1.0/GoogleMapsCompatible
///TopLeftCorner: -20037508.3427892 20037508.3427892

pub const WEB_MERCATOR_HALF_WORLD_WIDTH: f64 = 20_037_508.342_789_244;

/// 初期ズームレベル
pub const MAP_ZOOM_LEVEL: f64 = 5.0;

/// ズームレベルの制限
pub const MAP_MAX_ZOOM_LEVEL: f64 = 24.0;

pub const MAP_MIN_ZOOM_LEVEL: f64 = 0.0;

/// 1行分のホイールスクロールをピクセル量へ換算する係数
pub const MAP_SCROLL_LINE_DELTA_PIXELS: f32 = 20.0;

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
