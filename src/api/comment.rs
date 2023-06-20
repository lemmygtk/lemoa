use lemmy_api_common::{sensitive::Sensitive, comment::{CommentResponse, CreateComment}, lemmy_db_schema::newtypes::{PostId, CommentId}};


pub fn create_comment(
    post_id: i32,
    content: String,
    parent_id: Option<i32>,
    auth: Sensitive<String>,
) -> Result<CommentResponse, reqwest::Error> {
    let params = CreateComment {
        post_id: PostId(post_id),
        content,
        parent_id: parent_id.map(CommentId),
        auth,
        ..Default::default()
    };
    super::post("/comment", &params)
}
