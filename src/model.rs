use std::{
    collections::HashMap,
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
