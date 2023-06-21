use lemmy_api_common::{post::{GetPost, GetPostResponse, PostResponse, CreatePost, CreatePostLike, DeletePost}, lemmy_db_schema::{newtypes::{PostId, CommunityId}, CommentSortType, ListingType}, comment::{GetComments, GetCommentsResponse}, lemmy_db_views::structs::CommentView};

use crate::settings;

pub fn get_post(id: PostId) -> Result<GetPostResponse, reqwest::Error> {
    let params = GetPost {
        id: Some(id),
        auth: settings::get_current_account().jwt,
        ..Default::default()
    };

    super::get("/post", &params)
}

pub fn get_comments(post_id: PostId) -> Result<Vec<CommentView>, reqwest::Error> {
    let params = GetComments {
        post_id: Some(post_id),
        sort: Some(CommentSortType::Hot),
        type_: Some(ListingType::All),
        auth: settings::get_current_account().jwt,
        ..Default::default()
    };

    let mut comments = super::get::<GetCommentsResponse, _>("/comment/list", &params)?.comments;

    // hide removed and deleted comments
    comments.retain(|c| !c.comment.deleted && !c.comment.removed);

    Ok(comments)
}

pub fn default_post() -> GetPostResponse {
    serde_json::from_str(include_str!("../examples/post.json")).unwrap()
}

pub fn create_post(
    name: String,
    body: String,
    community_id: i32,
) -> Result<PostResponse, reqwest::Error> {
    let params = CreatePost {
        name,
        body: Some(body),
        community_id: CommunityId(community_id),
        auth: settings::get_current_account().jwt.unwrap(),
        ..Default::default()
    };
    super::post("/post", &params)
}

// for score, use 1 to upvote, -1 to vote down and 0 to reset the user's voting
pub fn like_post(post_id: PostId, score: i16) -> Result<PostResponse, reqwest::Error> {
    let params = CreatePostLike {
        post_id,
        score,
        auth: settings::get_current_account().jwt.unwrap(),
    };
    super::post("/post/like", &params)
}

pub fn delete_post(post_id: PostId) -> Result<PostResponse, reqwest::Error> {
    let params = DeletePost {
        post_id,
        deleted: true,
        auth: settings::get_current_account().jwt.unwrap(),
    };
    super::post("/post/delete", &params)
}
