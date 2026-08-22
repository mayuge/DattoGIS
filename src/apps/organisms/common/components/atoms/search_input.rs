use crate::domain::params::design_token_config::{
    BORDER_RADIUS_MD, COLOR_GRAY_20, COLOR_GRAY_50, HEADER_ICON_HEIGHT, SPACE_MD, SPACE_SM,
};
use crate::domain::params::text_config::SEARCH_PLACEHOLDER;
use gpui::*;
use gpui_component::input::{Input, InputEvent, InputState};
use std::path::PathBuf;

pub struct SearchSubmitted(pub String);

pub struct SearchInput {
    state: Entity<InputState>,
    _subscription: Subscription,
}

impl EventEmitter<SearchSubmitted> for SearchInput {}

impl SearchInput {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let state = cx.new(|cx| InputState::new(window, cx).placeholder(SEARCH_PLACEHOLDER));
        let _subscription = cx.subscribe(&state, |_, state, event: &InputEvent, cx| {
            if matches!(event, InputEvent::PressEnter { .. }) {
                let address = state.read(cx).value().to_string();
                if !address.trim().is_empty() {
                    cx.emit(SearchSubmitted(address));
                }
            }
        });

        Self {
            state,
            _subscription,
        }
    }
}

impl Render for SearchInput {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .relative()
            .top(px(SPACE_SM))
            .w(px(400.0))
            .child(
                Input::new(&self.state)
                    .w(px(400.0))
                    .border_1()
                    .border_color(rgb(COLOR_GRAY_50))
                    .rounded(px(BORDER_RADIUS_MD))
                    .bg(rgb(COLOR_GRAY_20))
                    .pl(px(HEADER_ICON_HEIGHT + SPACE_MD * 2.0))
                    .text_sm(),
            )
            .child(
                div()
                    .absolute()
                    .top(px(2.0))
                    .left(px(SPACE_MD))
                    .size(px(HEADER_ICON_HEIGHT))
                    .child(
                        img(PathBuf::from("assets/components/search.svg"))
                            .size(px(HEADER_ICON_HEIGHT)),
                    ),
            )
    }
}
