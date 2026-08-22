use crate::domain::params::design_token_config::{
    BORDER_WEIGHT, COLOR_COMPONENT_BASE, COLOR_GRAY_60, FOOTER_HEIGHT,
};
use gpui::*;

pub struct Footer {
    left_text: String,
    right_text: String,
}

impl Footer {
    /// 左右に表示するテキストを持つフッターを生成する。
    pub fn new(left_text: String, right_text: String) -> Self {
        Self {
            left_text,
            right_text,
        }
    }

    /// フッターのコンテナとステータステキストを描画する。
    pub fn render(self) -> impl IntoElement {
        div()
            .absolute()
            .bottom_0()
            .left_0()
            .h(px(FOOTER_HEIGHT))
            .w_full()
            .bg(rgb(COLOR_COMPONENT_BASE))
            .border_t(px(BORDER_WEIGHT))
            .border_color(rgb(COLOR_GRAY_60))
            .flex()
            .justify_between()
            .items_center()
            .child(div().text_xs().child(self.left_text))
            .child(div().text_xs().child(self.right_text))
    }
}
