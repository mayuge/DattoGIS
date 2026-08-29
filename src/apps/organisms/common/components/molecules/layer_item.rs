use gpui::*;

use crate::apps::organisms::common::components::atoms::checkbox::{Checkbox, CheckboxChanged};
use crate::domain::params::design_token_config::{
    BORDER_WEIGHT, COLOR_COMPONENT_BASE, COLOR_GRAY_60, COLOR_TEXT, HEADER_HEIGHT, SPACE_SM,
};

pub struct LayerItem {
    id: String,
    name: String,
    visible: bool,
    opacity: f32,
    checkbox: Entity<Checkbox>,
    _checkbox_subscription: Subscription,
}

pub struct LayerVisibilityChanged {
    pub id: String,
    pub visible: bool,
}

impl EventEmitter<LayerVisibilityChanged> for LayerItem {}

impl LayerItem {
    pub fn new(id: &str, name: &str, visible: bool, opacity: f32, cx: &mut Context<Self>) -> Self {
        let checkbox = cx.new(|_| Checkbox::new(visible));
        let checkbox_entity = checkbox.clone();
        let layer_id = id.to_string();
        let _checkbox_subscription = cx.subscribe(
            &checkbox_entity,
            move |layer_item, _, event: &CheckboxChanged, cx| {
                let new_visible = event.0;
                layer_item.visible = new_visible;
                cx.emit(LayerVisibilityChanged {
                    id: layer_id.clone(),
                    visible: new_visible,
                });
            },
        );

        Self {
            id: id.to_string(),
            name: name.to_string(),
            visible,
            opacity,
            checkbox,
            _checkbox_subscription,
        }
    }
}

impl Render for LayerItem {
    /// レイヤー名と表示状態のチェックボックスを描画する。
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w_full()
            .h(px(HEADER_HEIGHT))
            .px_2()
            .flex()
            .items_center()
            .gap(px(SPACE_SM))
            .text_xs()
            .border_b(px(BORDER_WEIGHT))
            .border_color(rgb(COLOR_GRAY_60))
            .bg(rgb(COLOR_COMPONENT_BASE))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(self.checkbox.clone()),
            )
            .child(div().text_color(rgb(COLOR_TEXT)).child(self.name.clone()))
    }
}
