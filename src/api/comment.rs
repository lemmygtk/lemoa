use lemmy_api_common::{comment::{CommentResponse, CreateComment, CreateCommentLike, DeleteComment, EditComment}, lemmy_db_schema::newtypes::{PostId, CommentId}};

use crate::settings;

pub fn create_comment(
    post_id: i32,
    content: String,
    parent_id: Option<i32>,
) -> Result<CommentResponse, reqwest::Error> {
    let params = CreateComment {
        post_id: PostId(post_id),
        content,
        parent_id: parent_id.map(CommentId),
        auth: settings::get_current_account().jwt.unwrap(),
        ..Default::default()
    };
    super::post("/comment", &params)
}

// see posts.rs for possible score parameters
pub fn like_comment(comment_id: CommentId, score: i16) -> Result<CommentResponse, reqwest::Error> {
    let params = CreateCommentLike {
        comment_id,
        score,
        auth: settings::get_current_account().jwt.unwrap(),
    };
    super::post("/comment/like", &params)
}

pub fn edit_comment(
    body: String,
    comment_id: i32
) -> Result<CommentResponse, reqwest::Error> {
    let params = EditComment {
        content: Some(body),
        comment_id: CommentId(comment_id),
        auth: settings::get_current_account().jwt.unwrap(),
        ..Default::default()
    };
    super::put("/post", &params)
}

pub fn delete_comment(comment_id: CommentId) -> Result<CommentResponse, reqwest::Error> {
    let params = DeleteComment {
        comment_id,
        deleted: true,
        auth: settings::get_current_account().jwt.unwrap(),
    };
    super::post("/comment/delete", &params)
}
