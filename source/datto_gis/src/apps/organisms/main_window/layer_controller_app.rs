use crate::apps::app::App as AppState;
use crate::domain::design_token_config::{
    BORDER_WEIGHT, COLOR_COMPONENT_BASE, COLOR_GRAY_60, LAYER_CONTROLLER_WIDTH,
};
use gpui::*;
pub struct LayerControllerApp;

impl LayerControllerApp {
    pub fn render(window: &mut Window, cx: &mut Context<AppState>) -> impl IntoElement {
        div()
            .absolute()
            .top_0()
            .left_0()
            .h_full()
            .border_color(rgb(COLOR_GRAY_60))
            .border_r(px(BORDER_WEIGHT))
            .w(px(LAYER_CONTROLLER_WIDTH))
            .bg(rgb(COLOR_COMPONENT_BASE))
    }
}
