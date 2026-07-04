use std::sync::Arc;

use anyhow::Result;
use futures::future::BoxFuture;
use gpui::*;
use gpui_http_client::{AsyncBody, HttpClient, Response};

use crate::apps::templates::main_window::MainTemplate;
use crate::domain::design_token_config::*;

struct ReqwestHttpClient {
    client: reqwest::blocking::Client,
    user_agent: http::HeaderValue,
}

impl ReqwestHttpClient {
    fn new() -> Self {
        let client = reqwest::blocking::Client::builder()
            .build()
            .expect("failed to create reqwest client");

        Self {
            client,
            user_agent: http::HeaderValue::from_static("datto_gis"),
        }
    }
}

impl HttpClient for ReqwestHttpClient {
    fn type_name(&self) -> &'static str {
        "ReqwestHttpClient"
    }

    fn user_agent(&self) -> Option<&http::HeaderValue> {
        Some(&self.user_agent)
    }

    fn send(&self, req: gpui_http_client::Request<AsyncBody>) -> BoxFuture<'static, Result<Response<AsyncBody>>> {
        let client = self.client.clone();
        let method = req.method().clone();
        let uri = req.uri().clone();
        let headers = req.headers().clone();

        Box::pin(async move {
            let url = reqwest::Url::parse(&uri.to_string())?;
            let mut request = reqwest::blocking::Request::new(method, url);
            *request.headers_mut() = headers;

            let response = client.execute(request)?;
            let status = response.status().as_u16();
            let headers = response.headers().clone();
            let bytes = response.bytes()?;
            let mut builder = http::Response::builder().status(status);

            for (name, value) in headers {
                if let Some(name) = name {
                    builder = builder.header(name, value.to_str().unwrap_or_default());
                }
            }

            let response = builder.body(AsyncBody::from_bytes(bytes.into()))?;
            Ok(response)
        })
    }

    fn proxy(&self) -> Option<&url::Url> {
        None
    }
}

pub struct App;

impl Render for App {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(COLOR_BASE))
            .text_color(rgb(COLOR_TEXT))
            .child(MainTemplate::render(window))
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
            |_window, cx| cx.new(|_| App),
        )
        .unwrap();
    });
}
