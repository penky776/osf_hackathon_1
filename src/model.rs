use std::{error::Error, path::Path};

use axum::headers::Cookie;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct AppState {
    pub data: Arc<Mutex<HashMap<String, String>>>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Post {
    pub post_id: Uuid,
    pub title: String,
    pub author: String,
    pub body: String,
    pub date: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Comment {
    pub comment_id: Uuid,
    pub post_id: Uuid,
    pub author: String,
    pub body: String,
    pub date: DateTime<Utc>,
}

pub fn get_time() -> DateTime<Utc> {
    Utc::now()
}

pub fn generate_unique_id() -> Uuid {
    Uuid::new_v4()
}

#[derive(Deserialize, Serialize)]
pub struct PostInput {
    pub title: String,
    pub body: String,
    pub csrf_token: String,
}

#[derive(Deserialize, Serialize)]
pub struct CommentInput {
    pub body: String,
    pub post_id: Uuid,
    pub csrf_token: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserInput {
    pub username: String,
    pub password: String,
}

pub fn write_to_json_file<P: AsRef<Path> + Clone, A: for<'a> Deserialize<'a> + Serialize>(
    path: P,
    input: A,
) -> Result<(), Box<dyn Error>> {
    let existing_json = std::fs::read_to_string(path.clone())?;

    let mut prev_data: Vec<A> =
        serde_json::from_str(&existing_json).expect("Failed to deserialize JSON data");

    prev_data.push(input);

    let updated_json = serde_json::to_string(&prev_data).expect("Failed to serialize data");
    std::fs::write(path, updated_json).expect("Failed to write data to file");

    Ok(())
}

pub fn remove_from_json_file_based_on_id<
    P: AsRef<Path> + Clone,
    A: for<'a> Deserialize<'a> + Serialize + ID,
>(
    path: P,
    id: Uuid,
) -> Result<(), Box<dyn Error>> {
    let exisiting_json = std::fs::read_to_string(path.clone())?;

    let mut prev_data: Vec<A> =
        serde_json::from_str(&exisiting_json).expect("Failed to deserialize JSON data");

    remove_object_with_id(&mut prev_data, id);

    let updated_json = serde_json::to_string(&prev_data).expect("Failed to serialize data");
    std::fs::write(path, updated_json).expect("Failed to write data to file");
    Ok(())
}

pub trait ID {
    fn get_id(&self) -> Uuid;
}

pub struct T {
    pub id: Uuid,
}

impl ID for Comment {
    fn get_id(&self) -> Uuid {
        self.comment_id.clone()
    }
}

impl ID for T {
    fn get_id(&self) -> Uuid {
        self.id.clone()
    }
}

impl ID for Post {
    fn get_id(&self) -> Uuid {
        self.post_id.clone()
    }
}

impl ID for User {
    fn get_id(&self) -> Uuid {
        self.user_id.clone()
    }
}

#[derive(Deserialize, Serialize)]
pub struct Id {
    pub id: Uuid,
    pub csrf_token: String,
}

fn remove_object_with_id<T: ID>(vector: &mut Vec<T>, id: Uuid) {
    let index = vector.iter().position(|x| x.get_id() == id).unwrap();
    vector.remove(index);
}

pub fn get_user_id<P: AsRef<Path>>(path: P, username: String) -> Result<Uuid, ()> {
    if let Ok(users) = std::fs::read_to_string(path) {
        if let Ok(users_vec) = serde_json::from_str::<Vec<User>>(&users) {
            for i in users_vec {
                if i.username == username {
                    return Ok(i.get_id());
                }
            }
        }
    }

    Err(())
}

pub fn check_csrf(state_original: AppState, cookie: Cookie, x_csrf_token: String) -> bool {
    let username = cookie.get("username").unwrap();
    let user_id = get_user_id("users.json", username.to_string())
        .unwrap()
        .to_string();

    let mut chars = x_csrf_token.chars();
    chars.next();
    chars.next_back();
    let x_csrf_token = chars.as_str().trim_matches('"');

    if let Ok(state_lock) = state_original.data.lock() {
        if let Some(key_token_pair) = state_lock.get_key_value(&user_id) {
            if key_token_pair.1 == x_csrf_token {
                eprintln!("csrf check returned true");
                return true;
            } else {
                eprintln!(
                    "csrf check returned false: this is in the hashmap -> {:?} and this is the token being compared -> {:?}",
                    key_token_pair.1, x_csrf_token
                );
            };
        } else {
            eprintln!("state_lock error 2");
        }
    }

    eprintln!("state_lock error");
    return false;
}
