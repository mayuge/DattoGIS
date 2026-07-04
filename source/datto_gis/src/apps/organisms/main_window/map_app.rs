use gpui::*;
use std::path::PathBuf;

pub struct MapApp;

impl MapApp {
    pub fn render(_window: &mut Window) -> impl IntoElement {
        div()
            .relative() // 基準
            .size_full()
            // 地図
            .child(div().size_full().bg(rgb(0xffffff)))
            // クロスヘア
            .child(
                img(PathBuf::from("assets/map/crosshair.svg"))
                    .absolute()
                    .top_1_2()
                    .left_1_2()
                    .w(px(32.0))
                    .h(px(32.0))
                    .ml(px(-16.0)) // 幅の半分
                    .mt(px(-16.0)), // 高さの半分
            )
    }
}
