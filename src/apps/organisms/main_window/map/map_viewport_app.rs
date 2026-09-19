use gpui::*;

use crate::apps::organisms::main_window::map::map_content_app::MapContent;

pub struct MapViewport {
    content: Entity<MapContent>,
    offset_x: f32,
    offset_y: f32,
}

impl MapViewport {
    pub fn new(content: Entity<MapContent>) -> Self {
        Self { content, offset_x: 0.0, offset_y: 0.0 }
    }

    pub fn set_offset(&mut self, offset_x: f32, offset_y: f32, cx: &mut Context<Self>) {
        self.offset_x = offset_x;
        self.offset_y = offset_y;
        cx.notify();
    }

    pub fn reset_offset(&mut self, cx: &mut Context<Self>) {
        self.set_offset(0.0, 0.0, cx);
    }
}

impl Render for MapViewport {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .absolute()
            .size_full()
            .top(px(self.offset_y))
            .left(px(self.offset_x))
            .child(self.content.clone())
    }
}