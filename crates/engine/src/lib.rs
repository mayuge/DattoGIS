
use proj_core::Transform;
use ogc_tms::{BoundingBox, TileMatrixSet};

pub struct MapXYZ {
    pub latitude: f64,
    pub longitude: f64,
    pub zoom: u32,
}

pub struct Screen {
    pub width: f64,
    pub height: f64,
}

pub struct TileZXY {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

pub struct MapObject {
    // WGS84: [longitude, latitude]
    pub center: [f64; 2],

    // WGS84: [min_lon, min_lat, max_lon, max_lat]
    pub bbox: [f64; 4],

    pub tiles: Vec<TileZXY>,

    // 最初のタイルの画面上の配置位置 (px)
    pub delta: [f64; 2],
}

/**
 * 入力された MapXYZ が有効かどうかを検証する関数
 * @param xyz 中心座標とズームレベルを持つマップの位置情報
 * @return bool 有効であれば true、無効であれば false
 */
pub fn validate_map_xyz(xyz: &MapXYZ) -> bool {
    // 緯度の範囲チェック
    if !(-90.0..=90.0).contains(&xyz.latitude) {
        return false;
    }
    // 経度の範囲チェック
    if !(-180.0..=180.0).contains(&xyz.longitude) {
        return false;
    }
    // ズームレベルの範囲チェック
    if xyz.zoom > 30 {
        return false;
    }
    true
}

/**
 * 入力されたscreen が有効かどうかを検証する関数
 * @param screen 画面の幅と高さ
 * @return bool 有効であれば true、無効であれば false
 */
pub fn validate_screen(screen: &Screen) -> bool {
    if !screen.width.is_finite() || !screen.height.is_finite() {
        return false;
    }
    if screen.width <= 0.0 || screen.height <= 0.0 {
        return false;
    }
    true
}


/**
 * マップオブジェクトを作成する関数
 * @param xyz 中心座標とズームレベルを持つマップの位置情報
 * @param screen 画面の幅と高さ
 * @return Result<MapObject, Box<dyn std::error::Error>> マップオブジェクトまたはエラー
 */
pub fn create_map_obj(
    xyz: MapXYZ,
    screen: Screen,
) -> Result<MapObject, Box<dyn std::error::Error>> {
    //緯度経度の範囲チェック
    if !validate_map_xyz(&xyz) {
        return Err("Invalid MapXYZ parameters".into());
    }
    // 画面の範囲チェック
    if !validate_screen(&screen) {
        return Err("Invalid Screen parameters".into());
    }

    let zoom = xyz.zoom as u8;

    // XYZ / WebMercatorQuad。標準の最大ズームは24。
    // 必要な場合はクレートの機能でズーム行列を拡張する。
    let base_tms = ogc_tms::web_mercator_quad();
    let tms: TileMatrixSet = base_tms.extended_to(zoom)?;

    // WGS84 <-> Web Mercator
    let forward = Transform::new("EPSG:4326", "EPSG:3857")?;
    let inverse = forward.inverse()?;

    // Web Mercator の有効な緯度範囲に制限
    let lat = xyz.latitude.clamp(
        -ogc_tms::MERCATOR_LAT_LIMIT,
        ogc_tms::MERCATOR_LAT_LIMIT,
    );

    let center = [xyz.longitude, lat];

    // 中心を投影座標 (m) に変換
    let (cx, cy) = forward.convert((center[0], center[1]))?;

    // タイル行列から解像度 (m/px) を取得
    let resolution = tms.resolution(zoom)?;

    // 画面に対応する投影座標上の範囲
    let half_w = screen.width * resolution / 2.0;
    let half_h = screen.height * resolution / 2.0;

    let left = cx - half_w;
    let right = cx + half_w;
    let bottom = cy - half_h;
    let top = cy + half_h;

    let projected_bbox = BoundingBox {
        left,
        bottom,
        right,
        top,
    };

    // 画面範囲に交差するタイルを列挙
    let tiles: Vec<TileZXY> = tms
        .tiles(projected_bbox, zoom..=zoom)?
        .map(|tile| -> Result<TileZXY, std::num::TryFromIntError> {
            Ok(TileZXY {
                x: u32::try_from(tile.x)?,
                y: u32::try_from(tile.y)?,
                z: u32::try_from(tile.z)?,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    // 左上の座標を含むタイルを特定
    let first_tile = tms.xy_tile(left, top, zoom)?;
    let first_bounds = tms.xy_bounds(first_tile)?;

    // 最初のタイルの画面上の配置位置 (px)
    let delta = [
        (first_bounds.left - left) / resolution,
        (top - first_bounds.top) / resolution,
    ];

    // 表示範囲の四隅を WGS84 に戻す
    let (min_lon, min_lat) = inverse.convert((left, bottom))?;
    let (max_lon, max_lat) = inverse.convert((right, top))?;

    let bbox = [
        min_lon,
        min_lat,
        max_lon,
        max_lat,
    ];

    Ok(MapObject {
        center,
        bbox,
        tiles,
        delta,
    })
}