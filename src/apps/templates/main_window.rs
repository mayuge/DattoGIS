use crate::apps::organisms::common::services::map::use_map_area::MapArea;
use crate::domain::params::app_config::*;
use crate::domain::traits::map_area_trait::MapAreaTrait;
use gpui::*;

use crate::apps::organisms::main_window::activity_bar_app::ActivityBarApp;
use crate::apps::organisms::main_window::footer_app::FooterApp;
use crate::apps::organisms::main_window::layer_controller_app::LayerControllerApp;
use crate::apps::organisms::main_window::map::map_app::MapApp;
use crate::apps::organisms::main_window::search_app::SearchApp;

use crate::apps::organisms::common::components::atoms::header::Header;
use std::path::PathBuf;

//MainWindowは、地図の初期状態とアプリ全体のUI構造を定義する
pub struct MainWindow {
    map_app: Entity<MapApp>,
    search_app: Entity<SearchApp>,
    footer_app: Entity<FooterApp>,
}

impl MainWindow {
    /// 各 organism を生成して画面を構成する。
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let map_app = cx.new(MapApp::new);
        let search_app = cx.new(|cx| SearchApp::new(window, map_app.clone(), cx));
        let footer_app = cx.new(|cx| FooterApp::new(map_app.clone(), cx));

        Self {
            map_app,
            search_app,
            footer_app,
        }
    }
}

impl Render for MainWindow {
    /// ウィンドウ内の各 organism と照準を配置する。
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> AnyElement {
        let viewport = window.viewport_size();
        let map_area =
            MapArea::get_map_area_size(f32::from(viewport.width), f32::from(viewport.height));
        let (crosshair_x, crosshair_y) = map_area.get_map_area_center();

        div()
            .relative()
            .size_full()
            .child(self.map_app.clone())
            .child(LayerControllerApp::render(window))
            .child(ActivityBarApp::render())
            .child(Header::new(APP_NAME.to_string()).render())
            .child(self.search_app.clone())
            .child(self.footer_app.clone())
            .child(
                img(PathBuf::from("assets/map/crosshair.svg"))
                    .absolute()
                    .top(px(crosshair_y))
                    .left(px(crosshair_x))
                    .w(px(32.0))
                    .h(px(32.0))
                    .ml(px(-16.0))
                    .mt(px(-16.0)),
            )
            .into_any()
    }
}
