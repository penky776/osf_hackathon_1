use argon2::Config;
use axum::{
    body::{boxed, Body, Full},
    debug_handler,
    extract::State,
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
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufReader, Write},
    path::Path,
    sync::{Arc, Mutex},
};
use tower::ServiceExt;
use tower_http::services::{ServeDir, ServeFile};

#[derive(Clone)]
struct AppState {
    data: Arc<Mutex<HashMap<String, String>>>,
}

#[tokio::main]
async fn main() {
    let shared_state: AppState = AppState {
        data: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/", get(home))
        .route("/addpost", post(add_post))
        .route("/deletepost", post(delete_post))
        .route("/addcomment", post(add_comment))
        .route("/deletecomment", post(delete_comment))
        .route("/deleteuser", delete(del_user))
        .nest_service("/static", ServeDir::new("assets/authenticated/static"))
        .route("/login", get(login).post(authenticate_login))
        .route("/register", get(register).post(authenticate_register))
        .with_state(shared_state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[debug_handler]
async fn home(
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

fn is_authenticated(state_original: AppState, cookie: Cookie) -> bool {
    let session_token = cookie.clone();
    let username = cookie;

    let auth = session_token.get("session_token");
    if let Some(cookie) = auth {
        let state = state_original.data.lock().unwrap();

        if let Some(user) = username.get("username") {
            if let Some(token) = state.get_key_value(user) {
                if cookie == token.1 {
                    return true;
                }
            }
        }
    }
    return false;
}

async fn login(
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

async fn register(
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

#[derive(Deserialize, Serialize, Debug, Clone)]
struct User {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Post {
    post_id: u32,
    title: String,
    author: String,
    body: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Comment {
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

async fn authenticate_login(
    State(state_original): State<AppState>,
    Form(user): Form<User>,
) -> impl IntoResponse {
    let u = read_user_from_file("users.json").unwrap();

    for a in &u {
        if a.username == user.username && a.password == user.password {
            let salt: [u8; 32] = rand::random();
            let config = Config::default();
            let token = argon2::hash_encoded(user.username.as_bytes(), &salt, &config).unwrap();

            let mut locked_state = state_original.data.lock().unwrap();
            locked_state.insert(user.username.clone(), token.clone());

            return Response::builder()
                .status(StatusCode::SEE_OTHER)
                .header(SET_COOKIE, "authenticated=yes")
                .header(SET_COOKIE, "session_token=".to_owned() + &token)
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

fn write_to_json_file<P: AsRef<Path> + Clone>(
    path: P,
    input: JsonData,
) -> Result<(), Box<dyn Error>> {
    let existing_json = std::fs::read_to_string(path.clone())?;

    match input {
        JsonData::Comment(comment) => {
            let mut prev_comments: Vec<Comment> =
                serde_json::from_str(&existing_json).expect("Failed to deserialize JSON data");

            prev_comments.push(comment);

            let updated_json =
                serde_json::to_string(&prev_comments).expect("Failed to serialize data");
            std::fs::write(path, updated_json).expect("failed to write data to file");
        }
        JsonData::Post(post) => {
            let mut prev_posts: Vec<Post> =
                serde_json::from_str(&existing_json).expect("Failed to deserialize JSON data");

            prev_posts.push(post);

            let updated_json =
                serde_json::to_string(&prev_posts).expect("Failed to serialize data");
            std::fs::write(path, updated_json).expect("failed to write data to file");
        }
        JsonData::User(user) => {
            let mut prev_users: Vec<User> =
                serde_json::from_str(&existing_json).expect("Failed to deserialize JSON data");

            prev_users.push(user);

            let updated_json =
                serde_json::to_string(&prev_users).expect("Failed to serialize data");
            std::fs::write(path, updated_json).expect("Failed to write data to file");
        }
    }
    Ok(())
}

async fn authenticate_register(
    State(state_original): State<AppState>,
    Form(user): Form<User>,
) -> impl IntoResponse {
    // TODO: hash passwords

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

    // create [USER].json and initialize it for storing json objects in an array
    let userjson_path =
        "assets/authenticated/static/api/json/users/".to_owned() + &user.username + ".json";
    let mut file = File::options()
        .append(true)
        .create(true)
        .open(userjson_path)
        .expect("Unable to create user's json file");
    writeln!(&mut file, "[]").unwrap();

    write_to_json_file("users.json", JsonData::User(user.clone())).unwrap();

    let salt: [u8; 32] = rand::random();
    let config = Config::default();
    let token = argon2::hash_encoded(user.username.as_bytes(), &salt, &config).unwrap();

    let mut locked_state = state_original.data.lock().unwrap();
    locked_state.insert(user.username.clone(), token.clone());

    return Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(SET_COOKIE, "authenticated=yes")
        .header(SET_COOKIE, "session_token=".to_owned() + &token)
        .header(SET_COOKIE, "username=".to_owned() + &user.username)
        .header(LOCATION, "/")
        .body(Full::from(
            "logged in with your new account! Please redirect to the homepage.",
        ))
        .unwrap();
}

#[derive(Deserialize, Serialize)]
struct PostInput {
    title: String,
    body: String,
}

async fn add_post(
    State(state_original): State<AppState>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Form(input): Form<PostInput>,
) {
    let username = cookie.get("username").expect("unable to find username");
    let posts_json_path = "assets/authenticated/static/api/json/posts.json";

    let post = construct_post(username, posts_json_path, input.title, input.body).unwrap();

    if is_authenticated(state_original, cookie) {
        write_to_json_file(posts_json_path, JsonData::Post(post.clone())).unwrap();
        write_to_json_file(
            "assets/authenticated/static/api/json/users/".to_owned() + &post.author + ".json",
            JsonData::Post(post),
        )
        .unwrap();
    }
}

fn construct_post<P: AsRef<Path>>(
    username: &str,
    path: P,
    post_title: String,
    post_body: String,
) -> Result<Post, Box<dyn Error>> {
    // read posts.json
    let existing_json = std::fs::read_to_string(path)?;

    let prev_posts: Vec<Post> =
        serde_json::from_str(&existing_json).expect("Failed to deserialize JSON data");

    let last_post: &Post = &prev_posts[(prev_posts.len() - 1).to_le()];
    let new_post_id = last_post.post_id + 1;

    let post = Post {
        post_id: new_post_id,
        title: post_title,
        author: username.to_string(),
        body: post_body,
    };

    Ok(post)
}

#[derive(Deserialize, Serialize)]
struct CommentInput {
    body: String,
    post_id: u32,
}

async fn add_comment(
    State(state_original): State<AppState>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Form(input): Form<CommentInput>,
) {
    // TODO
}

async fn delete_post() {}

async fn delete_comment() {}

async fn del_user() {}
