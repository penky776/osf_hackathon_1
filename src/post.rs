use std::error::Error;

use axum::{extract::State, headers::Cookie, Form, TypedHeader};

use crate::{
    authenticate::is_authenticated,
    model::{
        generate_unique_id, get_time, remove_from_json_file_based_on_id, write_to_json_file,
        AppState, Id, Post, PostInput,
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

pub fn construct_post(
    username: &str,
    post_title: String,
    post_body: String,
) -> Result<Post, Box<dyn Error>> {
    return Ok(Post {
        post_id: generate_unique_id(),
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
