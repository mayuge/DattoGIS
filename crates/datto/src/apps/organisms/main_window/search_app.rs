use crate::apps::organisms::common::components::atoms::search_input::{
    SearchInput, SearchSubmitted,
};
use crate::apps::organisms::main_window::map::map_app::MapApp;
use crate::domain::params::map_config::DATA_PROJ_EPSG;
use crate::domain::traits::coordinate_transformer_trait::CoordinateTransformer;
use crate::domain::traits::geocoding_trait::GeocodingTrait;
use crate::domain::types::map_coordinate_type::EpsgCoordinate;
use crate::infrastructure::coordinate::proj_core_coordinate_transformer::ProjCoreCoordinateTransformer;
use crate::infrastructure::geocoding::index::Geocoding;
use gpui::*;

pub struct SearchApp {
    search_input: Entity<SearchInput>,
    map_app: Entity<MapApp>,
    _search_subscription: Subscription,
    _search_task: Option<Task<()>>,
}

impl SearchApp {
    /// 検索入力と地図への更新先を初期化する。
    pub fn new(window: &mut Window, map_app: Entity<MapApp>, cx: &mut Context<Self>) -> Self {
        let search_input = cx.new(|cx| SearchInput::new(window, cx));
        let _search_subscription = cx.subscribe(&search_input, |search_app, _, event, cx| {
            let SearchSubmitted(address) = event;
            search_app.search(address.clone(), cx);
        });
        Self {
            search_input,
            map_app,
            _search_subscription,
            _search_task: None,
        }
    }

    /// 住所をジオコーディングし、取得した地点へ地図を移動する。
    fn search(&mut self, address: String, cx: &mut Context<Self>) {
        let map_app = self.map_app.clone();
        self._search_task = Some(cx.spawn(async move |_this, cx| {
            let result = cx
                .background_spawn(async move { Geocoding.search_address(&address) })
                .await;
            let Some(coordinate) = result.ok().flatten() else {
                return;
            };
            let Ok(center) =
                ProjCoreCoordinateTransformer.epsg_coordinate_to_web_mercator(EpsgCoordinate {
                    x: coordinate.longitude,
                    y: coordinate.latitude,
                    epsg: DATA_PROJ_EPSG,
                })
            else {
                return;
            };
            let _ = map_app.update(cx, |map_app, cx| map_app.set_center(center, cx));
        }));
    }
}

impl Render for SearchApp {
    /// 検索入力欄を中央に配置して描画する。
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .justify_center()
            .child(self.search_input.clone())
    }
}
