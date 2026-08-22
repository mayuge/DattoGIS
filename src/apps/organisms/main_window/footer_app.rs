use crate::apps::organisms::common::components::atoms::footer::Footer;
use crate::apps::organisms::main_window::map::map_app::{MapApp, MapChanged};
use crate::domain::params::map_config::DATA_PROJ_EPSG;
use crate::domain::traits::coordinate_transformer_trait::CoordinateTransformer;
use crate::infrastructure::coordinate::proj_core_coordinate_transformer::ProjCoreCoordinateTransformer;
use gpui::*;

pub struct FooterApp {
    map_app: Entity<MapApp>,
    _map_subscription: Subscription,
}

impl FooterApp {
    /// 地図状態の更新を購読するフッターを初期化する。
    pub fn new(map_app: Entity<MapApp>, cx: &mut Context<Self>) -> Self {
        let _map_subscription = cx.subscribe(&map_app, |_, _, _: &MapChanged, cx| cx.notify());
        Self {
            map_app,
            _map_subscription,
        }
    }

    /// 地図中心とズームをステータス表示用の文字列に変換する。
    fn map_status_text(&self, cx: &App) -> String {
        let map = self.map_app.read(cx).map();
        ProjCoreCoordinateTransformer
            .web_mercator_to_epsg_coordinate(map.center, DATA_PROJ_EPSG)
            .map(|coordinate| {
                format!(
                    "{:.4}, {:.4} | Zoom: {:.1} | EPSG: {}",
                    coordinate.x, coordinate.y, map.zoom_level, coordinate.epsg,
                )
            })
            .unwrap_or_else(|_| "座標変換エラー".to_string())
    }
}

impl Render for FooterApp {
    /// 現在の地図状態を含むフッターを描画する。
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Footer::new(String::new(), self.map_status_text(cx)).render()
    }
}
