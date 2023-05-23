use axum::{response::IntoResponse,  http::header::{self, HeaderMap, HeaderValue}};

const FAVICON: &[u8] = include_bytes!("../static/favicon.ico");


async fn asset(source: &'static [u8], ty: &'static str) -> impl IntoResponse {
    let mut headermap = HeaderMap::new();
    headermap.insert(header::CONTENT_TYPE, HeaderValue::from_static(ty));
    (headermap, source)
}

pub async fn favicon() -> impl IntoResponse {
	asset(FAVICON, "image/x-icon").await
}