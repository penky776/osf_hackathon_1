use axum::{
    body::Empty,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Form, Router,
};
use http::header::{LOCATION, SET_COOKIE};
use serde::Deserialize;
use std::{error::Error, fs::File, io::BufReader, path::Path};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/login", get(login).post(authenticate_login))
        .route("/register", get(register).post(authenticate_register));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn home() -> Html<&'static str> {
    Html(std::include_str!("../assets/authenticated/home.html"))
}

async fn login() -> Html<&'static str> {
    Html(std::include_str!("../assets/login.html"))
}

async fn register() -> Html<&'static str> {
    Html(std::include_str!("../assets/register.html"))
}

#[derive(Deserialize, Debug)]
struct Input {
    username: String,
    password: String,
}

#[derive(Deserialize, Debug)]
struct User {
    username: String,
    password: String,
}

fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<User>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let u: Vec<User> = serde_json::from_reader(reader)?;

    Ok(u)
}

async fn authenticate_login(Form(input): Form<Input>) -> impl IntoResponse {
    let u = read_user_from_file("users.json").unwrap();

    for user in &u {
        if user.username == input.username && user.password == input.password {
            return Response::builder()
                .status(StatusCode::SEE_OTHER)
                .header(SET_COOKIE, "authenticated=yes")
                .header(LOCATION, "/")
                .body(Empty::new())
                .unwrap();
        }
    }

    return Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(SET_COOKIE, "authenticated=no")
        .header(LOCATION, "/register")
        .body(Empty::new())
        .unwrap();
}

// TODO
async fn authenticate_register(Form(input): Form<Input>) -> impl IntoResponse {}
