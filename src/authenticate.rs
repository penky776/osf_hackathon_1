use std::{
    error::Error,
    fs::File,
    io::{BufReader, Write},
    path::Path,
};

use argon2::Config;
use axum::{
    body::Full,
    extract::State,
    headers::Cookie,
    response::{IntoResponse, Response},
    Form,
};
use http::{
    header::{LOCATION, SET_COOKIE},
    StatusCode,
};

use crate::model::{write_to_json_file, AppState, JsonData, User};

pub fn is_authenticated(state_original: AppState, cookie: Cookie) -> bool {
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

pub fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<User>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let u: Vec<User> = serde_json::from_reader(reader)?;

    Ok(u)
}

pub async fn authenticate_login(
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

pub async fn authenticate_register(
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
