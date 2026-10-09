/// 初期表示中心経度
pub const MAP_CENTER_LONGITUDE: f64 = 139.62286099196345;

/// 初期表示中心緯度
pub const MAP_CENTER_LATITUDE: f64 = 35.70538438612006;

/// 初期ズームレベル
pub const MAP_ZOOM_LEVEL: u32 = 18;

/// ズームレベルの制限
pub const MAP_MAX_ZOOM_LEVEL: u32 = 24;

pub const MAP_MIN_ZOOM_LEVEL: u32 = 0;

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
