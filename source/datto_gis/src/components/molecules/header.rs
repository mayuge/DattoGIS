use crate::domain::design_token_config::*;
use gpui::*;
use std::path::PathBuf;

pub struct Header {
    title: String,
}

impl Header {
    pub fn new(title: String) -> Self {
        Self { title }
    }

    pub fn render(&self, window: &mut Window) -> impl IntoElement {
        div()
            .flex()
            .window_control_area(WindowControlArea::Drag)
            .justify_between()
            .items_center()
            .h(px(HEADER_HEIGHT))
            .w_full()
            .bg(rgb(COLOR_GRAY_10))
            .px(px(SPACE_MD))
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(COLOR_TEXT))
                    .child(self.title.clone()),
            )
            .child(
                div()
                    .flex()
                    .gap(px(SPACE_SM))
                    .child(
                        div()
                            .size(px(24.0))
                            .flex()
                            .justify_center()
                            .items_center()
                            .child(
                                img(PathBuf::from("assets/window/minimize_window.svg"))
                                    .size(px(20.0)),
                            )
                            .on_mouse_down(MouseButton::Left, move |_, window, _| {
                                window.minimize_window();
                            }),
                    )
                    .child(
                        div()
                            .size(px(24.0))
                            .flex()
                            .justify_center()
                            .items_center()
                            .child(
                                img(PathBuf::from("assets/window/fullscreen_window.svg"))
                                    .size(px(20.0)),
                            )
                            .on_mouse_down(MouseButton::Left, move |_, window, _| {
                                window.zoom_window();
                            }),
                    )
                    .child(
                        div()
                            .size(px(24.0))
                            .flex()
                            .justify_center()
                            .items_center()
                            .child(
                                img(PathBuf::from("assets/window/close_window.svg")).size(px(20.0)),
                            )
                            .on_mouse_down(MouseButton::Left, move |_, window, _| {
                                window.remove_window();
                            }),
                    ),
            )
    }
}
