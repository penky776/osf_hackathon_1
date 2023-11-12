use axum::{
    body::{boxed, Body},
    extract::State,
    headers::Cookie,
    response::{Html, IntoResponse},
    Json, TypedHeader,
};
use http::Request;
use tower::ServiceExt;
use tower_http::services::ServeFile;

use crate::{
    authenticate::is_authenticated,
    model::{get_user_id, remove_from_json_file_based_on_id, AppState, User},
};

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

pub async fn del_user(
    State(state_original): State<AppState>,
    TypedHeader(cookie): TypedHeader<Cookie>,
) {
    let username = cookie.clone();
    let user_id = get_user_id("users.json", username.get("username").unwrap().to_string()).unwrap();

    if is_authenticated(state_original, cookie) {
        let userjson_path = "assets/authenticated/static/api/json/users/".to_owned()
            + username.get("username").unwrap()
            + ".json";

        std::fs::remove_file(userjson_path).unwrap();

        remove_from_json_file_based_on_id::<&str, User>("users.json", user_id).unwrap();
    }
}

pub async fn get_csrf_token(
    State(state_original): State<AppState>,
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> impl IntoResponse {
    let username = cookie.get("username").unwrap();
    let user_id = get_user_id("users.json", username.to_string())
        .unwrap()
        .to_string();

    return Json(
        state_original
            .data
            .lock()
            .unwrap()
            .get_key_value(&user_id)
            .unwrap()
            .1
            .clone(),
    );
}
