use crate::apps::templates::main_window::MainWindow;
use crate::domain::params::design_token_config::{
    BORDER_WEIGHT, COLOR_COMPONENT_BASE, COLOR_GRAY_60, LAYER_CONTROLLER_WIDTH,
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
            .border_color(rgb(COLOR_GRAY_60))
            .border_r(px(BORDER_WEIGHT))
            .w(px(LAYER_CONTROLLER_WIDTH))
            .bg(rgb(COLOR_COMPONENT_BASE))
    }
}
