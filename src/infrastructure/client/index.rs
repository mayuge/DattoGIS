use anyhow::Result;
use futures::future::BoxFuture;
use gpui::http_client::{AsyncBody, HttpClient, Response};
use crate::domain::params::app_config::{APP_NAME};

// ReqwestHttpClientは、reqwestを使用してHTTPリクエストを送信するための構造体
//インターネット画像をurl指定でのダウンロードに使う
pub struct ReqwestHttpClient {
    client: reqwest::blocking::Client,
    user_agent: http::HeaderValue,
}

impl ReqwestHttpClient {
    /// アプリケーション用の reqwest HTTP クライアントを生成する。
    pub fn new() -> Self {
        let client = reqwest::blocking::Client::builder()
            .build()
            .expect("failed to create reqwest client");

        Self {
            client,
            user_agent: http::HeaderValue::from_static(APP_NAME),
        }
    }
}

impl HttpClient for ReqwestHttpClient {
    /// HTTP リクエストで使用する User-Agent を返す。
    fn user_agent(&self) -> Option<&http::HeaderValue> {
        Some(&self.user_agent)
    }

    /// GPUI の HTTP リクエストを reqwest で送信する。
    fn send(&self, req: gpui::http_client::Request<AsyncBody>) -> BoxFuture<'static, Result<Response<AsyncBody>>> {
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

    /// プロキシ未使用であることを返す。
    fn proxy(&self) -> Option<&url::Url> {
        None
    }
}
