use gpui::*;
use gpui_component::{init as init_gpui_component, Root};
use gpui_platform::application;
use std::sync::Arc;

use crate::apps::templates::main_window::MainTemplate;
use crate::domain::params::design_token_config::{
    COLOR_BASE, COLOR_TEXT, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH, MIN_WINDOW_HEIGHT,
    MIN_WINDOW_WIDTH,
};

use crate::infrastructure::http::http_request_client::ReqwestHttpClient;

//appでは、アプリのルートコンポーネントを定義する
pub struct App {
    pub main_template: Entity<MainTemplate>,
}

impl App {
    //アプリのルートコンポーネントを作成する
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            //main_templateは、地図の初期状態とアプリ全体のUI構造を定義する
            main_template: cx.new(|cx| MainTemplate::new(window, cx)),
        }
    }
}

impl Render for App {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let main_template = self.main_template.clone();

        div()
            .size_full()
            .bg(rgb(COLOR_BASE))
            .text_color(rgb(COLOR_TEXT))
            .child(main_template.update(cx, |template, template_cx| {
                template.render(window, template_cx).into_any()
            }))
    }
}

/// アプリ起動
pub fn create_app() {
    application().run(move |cx| {
        init_gpui_component(cx);

        #[cfg(not(target_family = "wasm"))]
        {
            //httpリクエストを使えるようにする
            let http_client = ReqwestHttpClient::new();
            cx.set_http_client(Arc::new(http_client));
        }

        cx.open_window(
            WindowOptions {
                //デフォルトのタイトルバーを非表示にする
                titlebar: None,
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    //デフォルトのウィンドウサイズを設定する
                    size(px(DEFAULT_WINDOW_WIDTH), px(DEFAULT_WINDOW_HEIGHT)),
                    cx,
                ))),
                //ウィンドウの最小サイズを設定する
                window_min_size: Some(size(px(MIN_WINDOW_WIDTH), px(MIN_WINDOW_HEIGHT))),
                ..Default::default()
            },
            |window, cx| {
                //アプリのルートコンポーネントを作成する
                let app_view = cx.new(|cx| App::new(window, cx));
                //ルートコンポーネントを描画する
                cx.new(|cx| Root::new(app_view, window, cx))
            },
        )
        .unwrap();
    });
}
