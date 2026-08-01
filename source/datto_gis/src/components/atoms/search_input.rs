use crate::domain::params::design_token_config::{
    BORDER_RADIUS_MD, COLOR_GRAY_20, COLOR_GRAY_50, HEADER_ICON_HEIGHT, SPACE_MD,
};
use crate::domain::params::text_config::SEARCH_PLACEHOLDER;
use gpui::*;
use gpui_component::input::{Input, InputState};

pub struct SearchInput {
    state: Entity<InputState>,
}

impl SearchInput {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let state = cx.new(|cx| InputState::new(window, cx).placeholder(SEARCH_PLACEHOLDER));

        Self { state }
    }
}

impl Render for SearchInput {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        println!("render search");
        Input::new(&self.state)
            .w(px(400.0))
            .h(px(28.0))
            .border_1()
            .top(px(SPACE_MD))
            .border_color(rgb(COLOR_GRAY_50))
            .rounded(px(BORDER_RADIUS_MD))
            .bg(rgb(COLOR_GRAY_20))
            .pl(px(SPACE_MD))
            .text_sm()
    }
}
