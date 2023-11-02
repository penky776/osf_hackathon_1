use axum::{
    body::{boxed, Body, BoxBody, Full},
    headers::Cookie,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{delete, get, post},
    Form, Router, TypedHeader,
};
use http::{
    header::{LOCATION, SET_COOKIE},
    Request,
};
use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File, io::BufReader, path::Path};
use tower::ServiceExt;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/", get(home))
        .route("/addpost", post(add_post))
        .route("/deletepost", post(delete_post))
        .route("/addcomment", post(add_comment))
        .route("/deletecomment", post(delete_comment))
        .route("/deleteuser", delete(del_user))
        .nest_service("/static", ServeDir::new("assets/authenticated/static"))
        .route("/login", get(login).post(authenticate_login))
        .route("/register", get(register).post(authenticate_register));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn home(TypedHeader(authenticated): TypedHeader<Cookie>) -> Response<BoxBody> {
    let service_authenticated = ServeFile::new("assets/authenticated/home.html");
    let service_unauthenticated = ServeFile::new("assets/unauthenticated/not_authenticated.html");

    let auth = authenticated.get("authenticated");
    if let Some(cookie) = auth {
        if cookie == "yes" {
            let res = service_authenticated
                .oneshot(Request::new(Body::empty()))
                .await
                .unwrap();
            return res.map(boxed);
        }
    }

    let res = service_unauthenticated
        .oneshot(Request::new(Body::empty()))
        .await
        .unwrap();
    return res.map(boxed);
}

async fn login(TypedHeader(authenticated): TypedHeader<Cookie>) -> Html<&'static str> {
    let auth = authenticated.get("authenticated");
    if let Some(cookie) = auth {
        if cookie == "yes" {
            return Html(std::include_str!(
                "../assets/authenticated/already_logged_in.html"
            ));
        }
    }

    return Html(std::include_str!("../assets/unauthenticated/login.html"));
}

async fn register(TypedHeader(authenticated): TypedHeader<Cookie>) -> Html<&'static str> {
    let auth = authenticated.get("authenticated");
    if let Some(cookie) = auth {
        if cookie == "yes" {
            return Html(std::include_str!(
                "../assets/authenticated/already_logged_in.html"
            ));
        }
    }

    return Html(std::include_str!("../assets/unauthenticated/register.html"));
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct User {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Post {
    data_type: String,
    post_id: u32,
    title: String,
    author: String,
    body: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Comment {
    data_type: String,
    comment_id: u32,
    post_id: u32,
    author: String,
    body: String,
}

#[derive(Debug)]
enum JsonData {
    User(User),
    Post(Post),
    Comment(Comment),
}

fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<User>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let u: Vec<User> = serde_json::from_reader(reader)?;

    Ok(u)
}

async fn authenticate_login(Form(user): Form<User>) -> impl IntoResponse {
    let u = read_user_from_file("users.json").unwrap();

    for a in &u {
        if a.username == user.username && a.password == user.password {
            return Response::builder()
                .status(StatusCode::SEE_OTHER)
                .header(SET_COOKIE, "authenticated=yes")
                .header(SET_COOKIE, "username=".to_owned() + &user.username)
                .header(LOCATION, "/")
                .body(Full::from("Logged in!"))
                .unwrap();
        }
    }

    return Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(SET_COOKIE, "authenticated=no")
        .body(Full::from("incorrect username or password!"))
        .unwrap();
}

fn write_to_json_file<P: AsRef<Path>>(path: P, input: JsonData) -> Result<(), Box<dyn Error>> {
    let existing_json = std::fs::read_to_string(path)?;

    match input {
        JsonData::Comment(comment) => {
            let mut prev_comments: Vec<Comment> =
                serde_json::from_str(&existing_json).expect("Failed to deserialize JSON data");

            prev_comments.push(comment);

            let updated_json =
                serde_json::to_string(&prev_comments).expect("Failed to serialize data");
            std::fs::write(
                "assets/authenticated/static/api/json/comments.json",
                updated_json,
            )
            .expect("failed to write data to file");
        }
        JsonData::Post(post) => {
            let mut prev_posts: Vec<Post> =
                serde_json::from_str(&existing_json).expect("Failed to deserialize JSON data");

            prev_posts.push(post);

            let updated_json =
                serde_json::to_string(&prev_posts).expect("Failed to serialize data");
            std::fs::write(
                "assets/authenticated/static/api/json/posts.json",
                updated_json,
            )
            .expect("failed to write data to file");
        }
        JsonData::User(user) => {
            let mut prev_users: Vec<User> =
                serde_json::from_str(&existing_json).expect("Failed to deserialize JSON data");

            prev_users.push(user);

            let updated_json =
                serde_json::to_string(&prev_users).expect("Failed to serialize data");
            std::fs::write("users.json", updated_json).expect("Failed to write data to file");
        }
    }
    Ok(())
}

async fn authenticate_register(Form(user): Form<User>) -> impl IntoResponse {
    let u = read_user_from_file("users.json").unwrap();

    for a in &u {
        if a.username == user.username {
            return Response::builder()
                .status(StatusCode::SEE_OTHER)
                .header(SET_COOKIE, "authenticated=no")
                .body(Full::from(
                    "username already exists! Please choose a different one.",
                ))
                .unwrap();
        }
    }

    File::create(
        "assets/authenticated/static/api/json/users/".to_owned() + &user.username + ".json",
    )
    .expect("unable to create json file for user");
    write_to_json_file("users.json", JsonData::User(user.clone())).unwrap();

    return Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(SET_COOKIE, "authenticated=yes")
        .header(SET_COOKIE, "username=".to_owned() + &user.username)
        .header(LOCATION, "/")
        .body(Full::from(
            "logged in with your new account! Please redirect to the homepage.",
        ))
        .unwrap();
}

async fn add_post(Form(post): Form<Post>) {
    write_to_json_file(
        "assets/authenticated/static/api/json/posts.json",
        JsonData::Post(post.clone()),
    )
    .unwrap();
    write_to_json_file(
        "assets/authenticated/static/api/json/users/".to_owned() + &post.author + ".json",
        JsonData::Post(post),
    )
    .unwrap();
}

async fn add_comment(Form(comment): Form<Comment>) {
    write_to_json_file(
        "assets/authenticated/static/api/json/comments.json",
        JsonData::Comment(comment.clone()),
    )
    .unwrap();
    write_to_json_file(
        "assets/authenticated/static/api/json/users/".to_owned() + &comment.author + ".json",
        JsonData::Comment(comment),
    )
    .unwrap();
}

async fn delete_post() {}

async fn delete_comment() {}

async fn del_user() {}
