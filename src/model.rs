use std::{
    collections::HashMap,
    error::Error,
    path::Path,
    sync::{Arc, Mutex},
};

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
    pub post_id: u32,
    pub title: String,
    pub author: String,
    pub body: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Comment {
    pub comment_id: u32,
    pub post_id: u32,
    pub author: String,
    pub body: String,
}

#[derive(Debug)]
pub enum JsonData {
    User(User),
    Post(Post),
    Comment(Comment),
}

#[derive(Deserialize, Serialize)]
pub struct PostInput {
    pub title: String,
    pub body: String,
}

#[derive(Deserialize, Serialize)]
pub struct CommentInput {
    pub body: String,
    pub post_id: u32,
}

pub trait ID {
    fn get_id(&self) -> u32;
}

pub struct T {
    pub id: u32,
}

impl ID for Comment {
    fn get_id(&self) -> u32 {
        self.comment_id
    }
}

impl ID for T {
    fn get_id(&self) -> u32 {
        self.id
    }
}

impl ID for Post {
    fn get_id(&self) -> u32 {
        self.post_id
    }
}

#[derive(Deserialize, Serialize)]
pub struct Id {
    pub id: u32,
}

pub fn write_to_json_file<P: AsRef<Path> + Clone>(
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

pub fn remove_object_with_id<T: ID>(vector: &mut Vec<T>, id: u32) {
    let index = vector.iter().position(|x| x.get_id() == id).unwrap();
    vector.remove(index);
}
