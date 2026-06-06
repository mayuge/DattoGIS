//GISのドメイン

//地図は北を上にする
const ROTATE_2D_ANGLE = 0

//正面に地図を表示する
const ROTATE_3D_ANGLE = 90

// 中心とする座標 日本
const MAP_CENTER = [139.6917, 35.6895]

// 地図のズームレベル
const MAP_ZOOM_LEVEL = 8

// 表示する座標系 ウェブメルカトル
const DISPLAY_PROJ_EPSG = 3857

// データの座標系 WGS84
const DATA_PROJ_EPSG = 4326

//　デフォルトタイルのURL 国土地理院標準地図
const DEFAULT_RASTER_TILE_URL = "https://cyberjapandata.gsi.go.jp/xyz/std/{z}/{x}/{y}.png"

// デフォルトのファイル形式
const DEFAULT_FILE_FORMAT = "geojson"