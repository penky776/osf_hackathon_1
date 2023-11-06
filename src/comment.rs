use std::{error::Error, path::Path};

use axum::{extract::State, headers::Cookie, Form, TypedHeader};

use crate::{
    authenticate::is_authenticated,
    model::{
        remove_object_with_id, write_to_json_file, AppState, Comment, CommentInput, Id, JsonData,
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

        write_to_json_file(comments_json_path, JsonData::Comment(comment.clone())).unwrap();
    }
}

pub fn construct_comment<P: AsRef<Path>>(
    username: &str,
    path: P,
    post_id: u32,
    comment_body: String,
) -> Result<Comment, Box<dyn Error>> {
    // read comments.json
    let existing_json = std::fs::read_to_string(path)?;

    let prev_comments: Vec<Comment> =
        serde_json::from_str(&existing_json).expect("Failed to deserialize JSON data");

    if prev_comments.len() != 0 {
        let last_comment: &Comment = &prev_comments[(prev_comments.len() - 1).to_le()];
        let new_comment_id = last_comment.comment_id + 1;

        let comment = Comment {
            comment_id: new_comment_id,
            post_id,
            author: username.to_string(),
            body: comment_body,
        };

        return Ok(comment);
    }

    return Ok(Comment {
        comment_id: 1,
        post_id,
        author: username.to_string(),
        body: comment_body,
    });
}

pub async fn delete_comment(
    State(state_original): State<AppState>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    Form(comment_id): Form<Id>,
) {
    let comments_json = "assets/authenticated/static/api/json/comments.json";

    if is_authenticated(state_original, cookie) {
        let existing_json = std::fs::read_to_string(comments_json).unwrap();
        let mut comments: Vec<Comment> =
            serde_json::from_str(&existing_json).expect("Failed to deserialize JSON data");

        remove_object_with_id(&mut comments, comment_id.id);

        let updated_json = serde_json::to_string(&comments).expect("Failed to serialize data");
        std::fs::write(comments_json, updated_json).expect("failed to write data to file");
    }
}
