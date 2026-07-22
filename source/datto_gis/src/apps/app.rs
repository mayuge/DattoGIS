use gpui::*;
use std::sync::Arc;

use crate::apps::templates::main_window::MainTemplate;
use crate::domain::params::design_token_config::{COLOR_BASE, COLOR_TEXT};
use crate::domain::params::map_config::{MAP_CENTER_LATITUDE, MAP_CENTER_LONGITUDE};
use crate::domain::traits::coordinate_transformer_trait::CoordinateTransformer;
use crate::domain::types::map_coordinate::{WebMercatorCoordinate, Wgs84Coordinate};
use crate::infrastructure::coordinate::proj_core_coordinate_transformer::ProjCoreCoordinateTransformer;
use crate::infrastructure::http::http_request_client::ReqwestHttpClient;
use crate::services::map::use_map_instance::MapInstance;

pub struct App {
    pub map: MapInstance,
}

impl App {
    fn new(map_center: WebMercatorCoordinate) -> Self {
        Self {
            map: MapInstance::new(map_center),
        }
    }
}

impl Render for App {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(COLOR_BASE))
            .text_color(rgb(COLOR_TEXT))
            .child(MainTemplate::render(window, cx, self.map.clone()))
    }
}

/// アプリ起動
pub fn create_app() {
    let coordinate_transformer = ProjCoreCoordinateTransformer;
    let map_center = coordinate_transformer
        .wgs84_to_web_mercator(Wgs84Coordinate {
            longitude_deg: MAP_CENTER_LONGITUDE,
            latitude_deg: MAP_CENTER_LATITUDE,
        })
        .expect("failed to convert initial map center to Web Mercator");

    Application::new().run(move |cx| {
        #[cfg(not(target_family = "wasm"))]
        {
            let http_client = ReqwestHttpClient::new();
            cx.set_http_client(Arc::new(http_client));
        }

        cx.open_window(
            WindowOptions {
                titlebar: None,
                ..Default::default()
            },
            |_window, cx| cx.new(|_| App::new(map_center)),
        )
        .unwrap();
    });
}
