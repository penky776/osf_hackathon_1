use std::{error::Error, path::Path};

use axum::{extract::State, headers::Cookie, Form, TypedHeader};

use crate::{
    authenticate::is_authenticated,
    model::{
        get_time, remove_from_json_file_based_on_id, write_to_json_file, AppState, Id, Post,
        PostInput,
    },
};

pub async fn add_post(
    State(state_original): State<AppState>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Form(input): Form<PostInput>,
) {
    let username = cookie.clone();
    let posts_json_path = "assets/authenticated/static/api/json/posts.json";

    if is_authenticated(state_original, cookie) {
        let post = construct_post(
            username.get("username").expect("unable to find username"),
            posts_json_path,
            input.title,
            input.body,
        )
        .unwrap();

        write_to_json_file(posts_json_path, post.clone()).unwrap();
        write_to_json_file(
            "assets/authenticated/static/api/json/users/".to_owned() + &post.author + ".json",
            post,
        )
        .unwrap();
    }
}

pub fn construct_post<P: AsRef<Path>>(
    username: &str,
    path: P,
    post_title: String,
    post_body: String,
) -> Result<Post, Box<dyn Error>> {
    // read posts.json
    let existing_json = std::fs::read_to_string(path)?;

    let prev_posts: Vec<Post> =
        serde_json::from_str(&existing_json).expect("Failed to deserialize JSON data");

    // check if posts.json is empty
    if prev_posts.len() != 0 {
        let last_post: &Post = &prev_posts[(prev_posts.len() - 1).to_le()];
        let new_post_id = last_post.post_id.parse::<u32>().unwrap() + 1;

        let post = Post {
            post_id: new_post_id.to_string(),
            title: post_title,
            author: username.to_string(),
            body: post_body,
            date: get_time(),
        };

        return Ok(post);
    }

    return Ok(Post {
        post_id: 1.to_string(),
        title: post_title,
        author: username.to_string(),
        body: post_body,
        date: get_time(),
    });
}

pub async fn delete_post(
    State(state_original): State<AppState>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Form(post_id): Form<Id>,
) {
    let username_cookie = cookie.clone();
    let user = username_cookie.get("username").unwrap();

    let posts_json = "assets/authenticated/static/api/json/posts.json";
    let user_json = "assets/authenticated/static/api/json/users/".to_owned() + user + ".json";

    if is_authenticated(state_original, cookie) {
        remove_from_json_file_based_on_id::<&str, Post>(posts_json, post_id.id.clone()).unwrap();

        remove_from_json_file_based_on_id::<&str, Post>(&user_json, post_id.id).unwrap();
    }
}
