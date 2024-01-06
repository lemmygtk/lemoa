use lemmy_api_common::{
    comment::{CommentResponse, RemoveComment},
    lemmy_db_schema::newtypes::{CommentId, PostId},
    post::{PostResponse, RemovePost},
};

pub fn remove_post(post_id: i32, reason: String) -> Result<PostResponse, reqwest::Error> {
    let params = RemovePost {
        post_id: PostId(post_id),
        removed: true,
        reason: Some(reason),
    };
    super::post("/post/remove", &params)
}

pub fn remove_comment(comment_id: i32, reason: String) -> Result<CommentResponse, reqwest::Error> {
    let params = RemoveComment {
        comment_id: CommentId(comment_id),
        removed: true,
        reason: Some(reason),
    };
    super::post("/comment/remove", &params)
}
