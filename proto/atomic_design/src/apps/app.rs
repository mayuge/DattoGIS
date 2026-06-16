use gpui::*;

use crate::apps::templates::main_window::MainTemplate;
use crate::domain::app_config::*;
use crate::domain::design_token_config::*;

pub struct App;

impl Render for App {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(COLOR_BASE))
            .text_color(rgb(COLOR_TEXT))
            .child(MainTemplate::render())
    }
}

/// アプリ起動
pub fn create_app() {
    Application::new().run(|cx| {
        cx.open_window(
            WindowOptions {
                titlebar: None,
                ..Default::default()
            },
            |_window, cx| cx.new(|_| App),
        )
        .unwrap();
    });
}