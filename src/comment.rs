use std::error::Error;

use axum::{extract::State, headers::Cookie, Form, TypedHeader};
use uuid::Uuid;

use crate::{
    authenticate::is_authenticated,
    model::{
        generate_unique_id, get_time, remove_from_json_file_based_on_id, write_to_json_file,
        AppState, Comment, CommentInput, Id,
    },
};

pub async fn add_comment(
    State(state_original): State<AppState>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Form(input): Form<CommentInput>,
) {
    let username = cookie.clone();
    let comments_json_path = "assets/authenticated/static/api/json/comments.json";

    if is_authenticated(state_original, cookie) {
        let comment = construct_comment(
            username.get("username").expect("unable to find username"),
            input.post_id,
            input.body,
        )
        .unwrap();

        write_to_json_file(comments_json_path, comment).unwrap();
    }
}

pub fn construct_comment(
    username: &str,
    post_id: Uuid,
    comment_body: String,
) -> Result<Comment, Box<dyn Error>> {
    return Ok(Comment {
        comment_id: generate_unique_id(),
        post_id,
        author: username.to_string(),
        body: comment_body,
        date: get_time(),
    });
}

pub async fn delete_comment(
    State(state_original): State<AppState>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Form(comment_id): Form<Id>,
) {
    let comments_json = "assets/authenticated/static/api/json/comments.json";

    if is_authenticated(state_original, cookie) {
        remove_from_json_file_based_on_id::<&str, Comment>(comments_json, comment_id.id).unwrap();
    }
}
