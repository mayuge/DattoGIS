use crate::apps::organisms::common::components::molecules::layer_item::{
    LayerItem, LayerOpacityChanged, LayerVisibilityChanged,
};
use crate::domain::params::design_token_config::{
    ACTIVITY_BAR_WIDTH, BORDER_WEIGHT, COLOR_COMPONENT_BASE, COLOR_GRAY_60, HEADER_HEIGHT,
    LAYER_CONTROLLER_WIDTH, SPACE_MD,
};
use crate::domain::traits::load_raster_tile_config_trait::LoadRasterTileConfigTrait;
use crate::infrastructure::json::load_raster_tile_config::LoadRasterTileConfig;
use gpui::*;

pub struct LayerControllerApp {
    layers: Vec<Entity<LayerItem>>,
    _layer_subscriptions: Vec<Subscription>,
}

impl EventEmitter<LayerVisibilityChanged> for LayerControllerApp {}
impl EventEmitter<LayerOpacityChanged> for LayerControllerApp {}

impl LayerControllerApp {
    /// レイヤー操作パネルを初期化する。
    pub fn new(cx: &mut Context<Self>) -> Self {
        let layers: Vec<_> = LoadRasterTileConfig::load()
            .into_iter()
            .map(|layer| {
                cx.new(|cx| {
                    LayerItem::new(&layer.id, &layer.name, layer.visible, layer.opacity, cx)
                })
            })
            .collect();
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
            _layer_subscriptions,
        }
    }
}

impl Render for LayerControllerApp {
    /// レイヤー操作パネルを描画する。
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
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
            .children(self.layers.iter().cloned().collect::<Vec<_>>())
    }
}
