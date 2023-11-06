use std::{error::Error, path::Path};

use axum::{extract::State, headers::Cookie, Form, TypedHeader};

use crate::{
    authenticate::is_authenticated,
    model::{remove_object_with_id, write_to_json_file, AppState, Id, JsonData, Post, PostInput},
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
        write_to_json_file(posts_json_path, JsonData::Post(post.clone())).unwrap();
        write_to_json_file(
            "assets/authenticated/static/api/json/users/".to_owned() + &post.author + ".json",
            JsonData::Post(post),
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
        let new_post_id = last_post.post_id + 1;

        let post = Post {
            post_id: new_post_id,
            title: post_title,
            author: username.to_string(),
            body: post_body,
        };

        return Ok(post);
    }

    return Ok(Post {
        post_id: 1,
        title: post_title,
        author: username.to_string(),
        body: post_body,
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
        let existing_json = std::fs::read_to_string(posts_json).unwrap();
        let mut posts: Vec<Post> =
            serde_json::from_str(&existing_json).expect("Failed to deserialize JSON data");

        remove_object_with_id(&mut posts, post_id.id);

        let updated_json = serde_json::to_string(&posts).expect("Failed to serialize data");
        std::fs::write(posts_json, updated_json).expect("failed to write data to file");

        let existing_json_userjson = std::fs::read_to_string(&user_json).unwrap();
        let mut posts_userjson: Vec<Post> = serde_json::from_str(&existing_json_userjson).unwrap();

        remove_object_with_id(&mut posts_userjson, post_id.id);

        let updated_json_userjson =
            serde_json::to_string(&posts_userjson).expect("Failed to serialize data");
        std::fs::write(&user_json, updated_json_userjson).expect("failed to write data to file");
    }
}
