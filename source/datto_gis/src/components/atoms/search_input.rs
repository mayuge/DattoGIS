use crate::domain::design_token_config::{
    BORDER_RADIUS_MD, COLOR_GRAY_20, COLOR_GRAY_50, HEADER_HEIGHT, HEADER_ICON_HEIGHT, SPACE_SM,
};
use crate::domain::text_config::SEARCH_PLACEHOLDER;
use gpui::*;

pub struct SearchInput {
    text: String,
}

//検索用入力欄
impl SearchInput {
    pub fn new() -> Self {
        Self {
            text: String::new(),
        }
    }

    pub fn render(&self) -> impl IntoElement {
        div().h(px(HEADER_HEIGHT)).flex().items_center().child(
            div()
                .w(px(400.0))
                .h(px(HEADER_ICON_HEIGHT))
                .border_1()
                .border_color(rgb(COLOR_GRAY_50))
                .rounded(px(BORDER_RADIUS_MD))
                .bg(rgb(COLOR_GRAY_20))
                .flex()
                .items_center()
                .pl(px(SPACE_SM))
                .text_xs()
                .child(if self.text.is_empty() {
                    SEARCH_PLACEHOLDER.to_owned()
                } else {
                    self.text.clone()
                }),
        )
    }
}
