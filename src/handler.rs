use axum::{
    body::{boxed, Body},
    extract::State,
    headers::Cookie,
    response::{Html, IntoResponse},
    TypedHeader,
};
use http::Request;
use tower::ServiceExt;
use tower_http::services::ServeFile;

use crate::{authenticate::is_authenticated, model::AppState};

pub async fn login(
    State(state_original): State<AppState>,
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Html<&'static str> {
    if is_authenticated(state_original, cookie) {
        return Html(std::include_str!(
            "../assets/authenticated/already_logged_in.html"
        ));
    }

    return Html(std::include_str!("../assets/unauthenticated/login.html"));
}

pub async fn register(
    State(state_original): State<AppState>,
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Html<&'static str> {
    if is_authenticated(state_original, cookie) {
        return Html(std::include_str!(
            "../assets/authenticated/already_logged_in.html"
        ));
    }

    return Html(std::include_str!("../assets/unauthenticated/register.html"));
}

#[axum::debug_handler]
pub async fn home(
    State(state_original): State<AppState>,
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> impl IntoResponse {
    let service_authenticated = ServeFile::new("assets/authenticated/home.html");
    let service_unauthenticated = ServeFile::new("assets/unauthenticated/not_authenticated.html");

    if is_authenticated(state_original, cookie) {
        let res = service_authenticated
            .oneshot(Request::new(Body::empty()))
            .await
            .unwrap();
        return res.map(boxed);
    }

    let res = service_unauthenticated
        .oneshot(Request::new(Body::empty()))
        .await
        .unwrap();
    return res.map(boxed);
}

// TODO
pub async fn del_user() {}
