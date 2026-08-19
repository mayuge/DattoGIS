use crate::domain::params::design_token_config::{
    BORDER_WEIGHT, COLOR_COMPONENT_BASE, COLOR_DANGER, COLOR_GRAY_20, COLOR_GRAY_60, HEADER_HEIGHT,
    HEADER_ICON_HEIGHT, HEADER_ICON_SIZE, SPACE_MD, SPACE_SM,
};
use gpui::*;
use std::path::PathBuf;

pub struct Header {
    title: String,
}

impl Header {
    pub fn new(title: String) -> Self {
        Self { title }
    }

    pub fn render(&self) -> impl IntoElement {
        div()
            .absolute()
            .top_0()
            .left_0()
            .h(px(HEADER_HEIGHT))
            .flex()
            .window_control_area(WindowControlArea::Drag)
            .justify_between()
            .items_center()
            .w_full()
            .bg(rgb(COLOR_COMPONENT_BASE))
            .px(px(SPACE_MD))
            .border_b(px(BORDER_WEIGHT))
            .border_color(rgb(COLOR_GRAY_60))
            .child(
                div().child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(SPACE_MD))
                        .child(
                            img(PathBuf::from("assets/main_logo/datto_logo.svg"))
                                .size(px(HEADER_ICON_HEIGHT)),
                        )
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::BOLD)
                                .child(self.title.clone()),
                        ),
                ),
            )
            .child(
                div()
                    .flex()
                    .gap(px(SPACE_SM))
                    .child(
                        div()
                            .size(px(HEADER_ICON_HEIGHT))
                            .flex()
                            .justify_center()
                            .items_center()
                            .hover(|style| style.bg(rgb(COLOR_GRAY_20)))
                            .child(
                                img(PathBuf::from("assets/window/minimize_window.svg"))
                                    .size(px(HEADER_ICON_SIZE)),
                            )
                            .on_mouse_down(MouseButton::Left, |_, window, _| {
                                window.minimize_window();
                            }),
                    )
                    .child(
                        div()
                            .size(px(HEADER_ICON_HEIGHT))
                            .flex()
                            .justify_center()
                            .items_center()
                            .hover(|style| style.bg(rgb(COLOR_GRAY_20)))
                            .child(
                                img(PathBuf::from("assets/window/fullscreen_window.svg"))
                                    .size(px(HEADER_ICON_SIZE)),
                            )
                            .on_mouse_down(MouseButton::Left, |_, window, _| {
                                window.zoom_window();
                            }),
                    )
                    .child(
                        div()
                            .size(px(HEADER_ICON_HEIGHT))
                            .flex()
                            .justify_center()
                            .items_center()
                            .hover(|style| style.bg(rgb(COLOR_DANGER)))
                            .child(
                                img(PathBuf::from("assets/window/close_window.svg"))
                                    .size(px(HEADER_ICON_SIZE)),
                            )
                            .on_mouse_down(MouseButton::Left, |_, window, _| {
                                window.remove_window();
                            }),
                    ),
            )
    }
}
