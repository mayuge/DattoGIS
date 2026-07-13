use crate::domain::design_token_config::{
    ACTIVITY_BAR_ICON_SIZE, ACTIVITY_BAR_WIDTH, BORDER_WEIGHT, COLOR_COMPONENT_BASE, COLOR_GRAY_20,
    COLOR_GRAY_60, HEADER_HEIGHT, SPACE_MD,
};
use gpui::*;
use std::path::PathBuf;

pub struct ActivityBarApp;

impl ActivityBarApp {
    pub fn render() -> impl IntoElement {
        div()
            .absolute()
            .h_full()
            .w(px(ACTIVITY_BAR_WIDTH))
            .pt(px(HEADER_HEIGHT + SPACE_MD))
            .left_0()
            .flex()
            .flex_col()
            .items_center()
            .border_r(px(BORDER_WEIGHT))
            .border_color(rgb(COLOR_GRAY_60))
            .gap(px(SPACE_MD))
            .child(
                img(PathBuf::from("assets/activity_bar/file.svg"))
                    .size(px(ACTIVITY_BAR_ICON_SIZE))
                    .hover(|style| style.bg(rgb(COLOR_GRAY_20))),
            )
            .child(
                img(PathBuf::from("assets/activity_bar/layer.svg"))
                    .size(px(ACTIVITY_BAR_ICON_SIZE))
                    .hover(|style| style.bg(rgb(COLOR_GRAY_20))),
            )
    }
}
