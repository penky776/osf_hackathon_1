use authenticate::{authenticate_login, authenticate_register};
use axum::{
    routing::{delete, get, post},
    Router,
};
use comment::{add_comment, delete_comment};
use handler::{del_user, get_csrf_token, home, login, register};
use model::AppState;
use post::{add_post, delete_post};
use tower_http::services::ServeDir;

mod authenticate;
mod comment;
mod handler;
mod model;
mod post;

#[tokio::main]
async fn main() {
    let shared_state: AppState = AppState::new();

    let app = Router::new()
        .route("/", get(home))
        .route("/addpost", post(add_post))
        .route("/deletepost", post(delete_post))
        .route("/addcomment", post(add_comment))
        .route("/deletecomment", post(delete_comment))
        .route("/deleteuser", delete(del_user).post(del_user))
        .nest_service("/static", ServeDir::new("assets/authenticated/static"))
        .route("/login", get(login).post(authenticate_login))
        .route("/register", get(register).post(authenticate_register))
        .route("/get-csrf-token", get(get_csrf_token))
        .with_state(shared_state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
