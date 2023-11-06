use std::{error::Error, path::Path};

use axum::{extract::State, headers::Cookie, Form, TypedHeader};

use crate::{
    authenticate::is_authenticated,
    model::{
        get_time, remove_from_json_file_based_on_id, write_to_json_file, AppState, Comment,
        CommentInput, Id,
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
            comments_json_path,
            input.post_id,
            input.body,
        )
        .unwrap();

        write_to_json_file(comments_json_path, comment).unwrap();
    }
}

pub fn construct_comment<P: AsRef<Path>>(
    username: &str,
    path: P,
    post_id: String,
    comment_body: String,
) -> Result<Comment, Box<dyn Error>> {
    // read comments.json
    let existing_json = std::fs::read_to_string(path)?;

    let prev_comments: Vec<Comment> =
        serde_json::from_str(&existing_json).expect("Failed to deserialize JSON data");

    if prev_comments.len() != 0 {
        let last_comment: &Comment = &prev_comments[(prev_comments.len() - 1).to_le()];
        let new_comment_id = last_comment.comment_id.parse::<u32>().unwrap() + 1;

        let comment = Comment {
            comment_id: new_comment_id.to_string(),
            post_id,
            author: username.to_string(),
            body: comment_body,
            date: get_time(),
        };

        return Ok(comment);
    }

    return Ok(Comment {
        comment_id: 1.to_string(),
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
