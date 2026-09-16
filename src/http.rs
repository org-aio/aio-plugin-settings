use aio_plugin_identity_model::SessionView;
use gloo_net::http::Request;
use serde::Deserialize;

#[derive(Deserialize)]
struct Response<T> {
    data: T,
}

pub(super) async fn load() -> Result<SessionView, String> {
    get::<Option<SessionView>>("/api/auth/session")
        .await?
        .ok_or_else(|| "会话已失效，请重新登录".to_owned())
}

pub(super) async fn get<T: for<'de> Deserialize<'de>>(path: &str) -> Result<T, String> {
    let response = Request::get(path).send().await.map_err(|e| e.to_string())?;
    if !response.ok() {
        return Err(error(response).await);
    }
    response
        .json::<Response<T>>()
        .await
        .map(|r| r.data)
        .map_err(|e| e.to_string())
}

async fn error(response: gloo_net::http::Response) -> String {
    let body = response.text().await.unwrap_or_default();
    serde_json::from_str::<serde_json::Value>(&body)
        .ok()
        .and_then(|v| v.get("error")?.as_str().map(str::to_owned))
        .unwrap_or(body)
}
