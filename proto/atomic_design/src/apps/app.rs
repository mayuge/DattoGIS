use gpui::*;

use crate::domain::app_config::*;
use crate::domain::design_token_config::*;
use crate::apps::templates::home::HomeTemplate;

pub struct App;

impl Render for App {
    fn render(
        &mut self,
        window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        window.set_window_title(&format!(
            "{} v{}",
            APP_NAME,
            APP_VERSION
        ));

        div()
            .size_full()
            .bg(rgb(COLOR_BASE))
            .text_color(rgb(COLOR_TEXT))
            .child(HomeTemplate::render())
    }
}

/// アプリ起動
pub fn create_app() {
    Application::new().run(|cx| {
        cx.open_window(
            WindowOptions::default(),
            |_window, cx| cx.new(|_| App),
        )
        .unwrap();
    });
}