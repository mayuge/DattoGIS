use gpui::*;

use crate::domain::params::design_token_config::{
    BORDER_WEIGHT, COLOR_COMPONENT_BASE, COLOR_GRAY_60, COLOR_TEXT, HEADER_HEIGHT,
};

pub struct LayerItem {
    id: String,
    name: String,
    visible: bool,
    opacity: f32,
}

impl LayerItem {
    /// レイヤーの表示情報からリスト項目を生成する。
    pub fn new(id: &str, name: &str, visible: bool, opacity: f32) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            visible,
            opacity,
        }
    }

    /// レイヤー名を表示するリスト項目を描画する。
    pub fn render(&self) -> impl IntoElement {
        div()
            .w_full()
            .h(px(HEADER_HEIGHT))
            .px_2()
            .flex()
            .items_center()
            .text_xs()
            .border_b(px(BORDER_WEIGHT))
            .border_color(rgb(COLOR_GRAY_60))
            .bg(rgb(COLOR_COMPONENT_BASE))
            .child(div().text_color(rgb(COLOR_TEXT)).child(self.name.clone()))
    }
}
