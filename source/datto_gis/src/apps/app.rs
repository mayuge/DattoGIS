use gpui::*;
use gpui_component::{init as init_gpui_component, Root};
use gpui_platform::application;
use std::sync::Arc;

use crate::apps::templates::main_window::MainTemplate;
use crate::domain::params::design_token_config::{
    COLOR_BASE, COLOR_TEXT, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH, MIN_WINDOW_HEIGHT,
    MIN_WINDOW_WIDTH,
};
use crate::domain::params::map_config::{
    DATA_PROJ_EPSG, DEFAULT_RASTER_TILE_URL, MAP_CENTER_LATITUDE, MAP_CENTER_LONGITUDE,
};
use crate::domain::traits::coordinate_transformer_trait::CoordinateTransformer;
use crate::domain::types::map_coordinate_type::{EpsgCoordinate, WebMercatorCoordinate};
use crate::domain::types::map_layer_type::RasterTileLayer;
use crate::infrastructure::coordinate::proj_core_coordinate_transformer::ProjCoreCoordinateTransformer;
use crate::infrastructure::http::http_request_client::ReqwestHttpClient;
use crate::services::map::use_map_instance::MapInstance;

pub struct App {
    pub map: MapInstance,
    pub raster_tile_layers: Vec<RasterTileLayer>,
}

impl App {
    fn new(map_center: WebMercatorCoordinate) -> Self {
        Self {
            map: MapInstance::new(map_center),
            raster_tile_layers: vec![
                RasterTileLayer {
                    id: "gsi".to_string(),
                    name: "地理院地図".into(),
                    url: DEFAULT_RASTER_TILE_URL.into(),
                    opacity: 0.5,
                    visible: true,
                    attribution: Some("地理院タイル".into()),
                },
                RasterTileLayer {
                    id: "gsi-photo".to_string(),
                    name: "航空写真".into(),
                    url: "https://cyberjapandata.gsi.go.jp/xyz/seamlessphoto/{z}/{x}/{y}.jpg"
                        .into(),
                    opacity: 0.6,
                    visible: true,
                    attribution: Some("地理院タイル".into()),
                },
            ],
        }
    }
}

impl Render for App {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(COLOR_BASE))
            .text_color(rgb(COLOR_TEXT))
            .child(MainTemplate::render(
                window,
                cx,
                self.map.clone(),
                self.raster_tile_layers.clone(),
            ))
    }
}

/// アプリ起動
pub fn create_app() {
    let coordinate_transformer = ProjCoreCoordinateTransformer;
    let map_center = coordinate_transformer
        .epsg_coordinate_to_web_mercator(EpsgCoordinate {
            x: MAP_CENTER_LONGITUDE,
            y: MAP_CENTER_LATITUDE,
            epsg: DATA_PROJ_EPSG,
        })
        .expect("failed to convert initial map center to Web Mercator");

    application().run(move |cx| {
        init_gpui_component(cx);

        #[cfg(not(target_family = "wasm"))]
        {
            let http_client = ReqwestHttpClient::new();
            cx.set_http_client(Arc::new(http_client));
        }

        cx.open_window(
            WindowOptions {
                titlebar: None,
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(DEFAULT_WINDOW_WIDTH), px(DEFAULT_WINDOW_HEIGHT)),
                    cx,
                ))),
                window_min_size: Some(size(px(MIN_WINDOW_WIDTH), px(MIN_WINDOW_HEIGHT))),
                ..Default::default()
            },
            |window, cx| {
                let app_view = cx.new(|_| App::new(map_center));
                cx.new(|cx| Root::new(app_view, window, cx))
            },
        )
        .unwrap();
    });
}
