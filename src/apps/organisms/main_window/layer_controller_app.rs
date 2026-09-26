use crate::apps::organisms::common::components::molecules::layer_item::{
    LayerItem, LayerOpacityChanged, LayerVisibilityChanged,
};
use crate::domain::params::design_token_config::{
    ACTIVITY_BAR_WIDTH, BORDER_WEIGHT, COLOR_COMPONENT_BASE, COLOR_GRAY_60, COLOR_TEXT,
    HEADER_HEIGHT, LAYER_CONTROLLER_WIDTH,
};
use crate::domain::params::text_config::BASE_MAP_TEXT;
use crate::domain::traits::load_raster_tile_json_trait::LoadRasterTileJsonTrait;
use crate::domain::traits::load_vector_json_trait::LoadVectorJsonTrait;
use crate::infrastructure::json::load_raster_tile_json::LoadRasterTileJson;
use crate::infrastructure::json::load_vector_json::LoadVectorJson;
use gpui::*;

pub struct LayerControllerApp {
    layers: Vec<Entity<LayerItem>>,
    vector_layer_count: usize,
    _layer_subscriptions: Vec<Subscription>,
}

impl EventEmitter<LayerVisibilityChanged> for LayerControllerApp {}
impl EventEmitter<LayerOpacityChanged> for LayerControllerApp {}

impl LayerControllerApp {
    /// レイヤー操作パネルを初期化する。
    pub fn new(cx: &mut Context<Self>) -> Self {
        // レイヤーの設定を取得する
        let vector_layers: Vec<_> = LoadVectorJson::load()
            .into_iter()
            .rev()
            .map(|layer| {
                cx.new(|cx| {
                    LayerItem::new(&layer.id, &layer.name, layer.visible, layer.opacity, cx)
                })
            })
            .collect();
        let vector_layer_count = vector_layers.len();
        let mut layers = vector_layers;
        layers.extend(LoadRasterTileJson::load().into_iter().map(|layer| {
            cx.new(|cx| LayerItem::new(&layer.id, &layer.name, layer.visible, layer.opacity, cx))
        }));
        let _layer_subscriptions = layers
            .iter()
            .flat_map(|layer| {
                let visibility_subscription =
                    cx.subscribe(layer, |_, _, event: &LayerVisibilityChanged, cx| {
                        cx.emit(LayerVisibilityChanged {
                            id: event.id.clone(),
                            visible: event.visible,
                        });
                    });
                let opacity_subscription =
                    cx.subscribe(layer, |_, _, event: &LayerOpacityChanged, cx| {
                        cx.emit(LayerOpacityChanged {
                            id: event.id.clone(),
                            opacity: event.opacity,
                        });
                    });
                [visibility_subscription, opacity_subscription]
            })
            .collect();

        Self {
            layers,
            vector_layer_count,
            _layer_subscriptions,
        }
    }
}

impl Render for LayerControllerApp {
    /// レイヤー操作パネルを描画する。
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let (vector_layers, raster_layers) = self.layers.split_at(self.vector_layer_count);

        div()
            .absolute()
            .top_0()
            .left_0()
            .h_full()
            .pt(px(HEADER_HEIGHT))
            .pl(px(ACTIVITY_BAR_WIDTH))
            .border_color(rgb(COLOR_GRAY_60))
            .border_r(px(BORDER_WEIGHT))
            .w(px(LAYER_CONTROLLER_WIDTH))
            .bg(rgb(COLOR_COMPONENT_BASE))
            .children(vector_layers.iter().cloned().collect::<Vec<_>>())
            .child(
                div()
                    .w_full()
                    .px_2()
                    .py_1()
                    .text_xs()
                    .text_color(rgb(COLOR_TEXT))
                    .child(BASE_MAP_TEXT)
                    .border_color(rgb(COLOR_GRAY_60))
                    .border_b(px(BORDER_WEIGHT)),
            )
            .children(raster_layers.iter().cloned().collect::<Vec<_>>())
    }
}
