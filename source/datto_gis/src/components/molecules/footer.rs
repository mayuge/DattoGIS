use crate::domain::design_token_config::*;
use gpui::*;

pub struct Footer {
    text: SharedString,
}

impl Footer {
    pub fn new(text: SharedString) -> Self {
        Self { text }
    }

    pub fn render(&self, _window: &mut Window) -> impl IntoElement {
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
            .items_center()
            .child(
                div()
                    .absolute()
                    .text_xs()
                    .right(px(2.0))
                    .child(self.text.clone()),
            )
    }
}
