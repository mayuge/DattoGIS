use crate::apps::templates::main_window::MainWindow;
use crate::apps::organisms::common::components::molecules::layer_item::LayerItem;
use crate::domain::params::design_token_config::{
    ACTIVITY_BAR_WIDTH, BORDER_WEIGHT, COLOR_COMPONENT_BASE, COLOR_GRAY_60, HEADER_HEIGHT,
    LAYER_CONTROLLER_WIDTH, SPACE_MD,
};
use gpui::*;
pub struct LayerControllerApp;

impl LayerControllerApp {
    pub fn render(_window: &mut Window, _cx: &mut Context<MainWindow>) -> impl IntoElement {
        div()
            .absolute()
            .top_0()
            .left_0()
            .h_full()
            .pt(px(HEADER_HEIGHT + SPACE_MD))
            .pl(px(ACTIVITY_BAR_WIDTH))
            .border_color(rgb(COLOR_GRAY_60))
            .border_r(px(BORDER_WEIGHT))
            .w(px(LAYER_CONTROLLER_WIDTH))
            .bg(rgb(COLOR_COMPONENT_BASE))
            .children(vec![
                LayerItem::new("layer1", "Layer 1", true, 1.0).render(),
                LayerItem::new("layer2", "Layer 2", false, 0.5).render(),
            ])
    }
}
