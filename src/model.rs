use std::{
    collections::HashMap,
    error::Error,
    path::Path,
    sync::{Arc, Mutex},
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AppState {
    pub data: Arc<Mutex<HashMap<String, String>>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Post {
    pub post_id: String,
    pub title: String,
    pub author: String,
    pub body: String,
    pub date: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Comment {
    pub comment_id: String,
    pub post_id: String,
    pub author: String,
    pub body: String,
    pub date: DateTime<Utc>,
}

pub fn get_time() -> DateTime<Utc> {
    Utc::now()
}

pub fn generate_unique_id() {
    // TODO
}

#[derive(Deserialize, Serialize)]
pub struct PostInput {
    pub title: String,
    pub body: String,
}

#[derive(Deserialize, Serialize)]
pub struct CommentInput {
    pub body: String,
    pub post_id: String,
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
    id: String,
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
    fn get_id(&self) -> String;
}

pub struct T {
    pub id: String,
}

impl ID for Comment {
    fn get_id(&self) -> String {
        self.comment_id.clone()
    }
}

impl ID for T {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl ID for Post {
    fn get_id(&self) -> String {
        self.post_id.clone()
    }
}

#[derive(Deserialize, Serialize)]
pub struct Id {
    pub id: String,
}

fn remove_object_with_id<T: ID>(vector: &mut Vec<T>, id: String) {
    let index = vector.iter().position(|x| x.get_id() == id).unwrap();
    vector.remove(index);
}
