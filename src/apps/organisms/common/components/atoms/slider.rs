use gpui::{
    AppContext, Context, Entity, EventEmitter, IntoElement, Render, Styled, Subscription, Window,
    rgb,
};
use gpui_component::slider::{Slider as ComponentSlider, SliderEvent, SliderState};

use crate::domain::params::design_token_config::{COLOR_GRAY_70, COLOR_TEXT};

pub struct SliderChanged(pub f32);

pub struct Slider {
    state: Entity<SliderState>,
    _subscription: Subscription,
}

impl EventEmitter<SliderChanged> for Slider {}

impl Slider {
    pub fn new(value: f32, cx: &mut Context<Self>) -> Self {
        let state = cx.new(|_| {
            SliderState::new()
                .min(0.0)
                .max(1.0)
                .step(0.1)
                .default_value(value.clamp(0.0, 1.0))
        });

        let _subscription = cx.subscribe(&state, |_, _state, event: &SliderEvent, cx| {
            if let SliderEvent::Change(value) = event {
                cx.emit(SliderChanged(value.start()));
            }
        });

        Self {
            state,
            _subscription,
        }
    }
}

impl Render for Slider {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        ComponentSlider::new(&self.state)
            .w_full()
            .bg(rgb(COLOR_GRAY_70))
            .text_color(rgb(COLOR_TEXT))
    }
}
