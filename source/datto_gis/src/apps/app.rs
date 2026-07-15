use gpui::*;
use std::sync::Arc;

use crate::apps::templates::main_window::MainTemplate;
use crate::domain::design_token_config::*;
use crate::infrastructure::http::http_request_client::ReqwestHttpClient;

#[derive(Default)]
pub struct App {
    pub coordinate: SharedString,
}

impl Render for App {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(COLOR_BASE))
            .text_color(rgb(COLOR_TEXT))
            .child(MainTemplate::render(window, cx, self.coordinate.clone()))
    }
}

/// アプリ起動
pub fn create_app() {
    Application::new().run(|cx| {
        #[cfg(not(target_family = "wasm"))]
        {
            let http_client = ReqwestHttpClient::new();
            cx.set_http_client(Arc::new(http_client));
        }

        cx.open_window(
            WindowOptions {
                titlebar: None,
                ..Default::default()
            },
            |_window, cx| cx.new(|_| App::default()),
        )
        .unwrap();
    });
}
