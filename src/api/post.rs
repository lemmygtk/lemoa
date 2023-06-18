use lemmy_api_common::{post::{GetPost, GetPostResponse}, lemmy_db_schema::{newtypes::PostId, CommentSortType, ListingType}, comment::{GetComments, GetCommentsResponse}, lemmy_db_views::structs::CommentView};

use crate::components::CLIENT;

pub fn get_post(id: PostId) -> std::result::Result<GetPostResponse, reqwest::Error> {
    let params = GetPost {
        id: Some(id),
        ..Default::default()
    };

    let url = format!("{}/post", super::get_api_url());
    CLIENT.get(&url).query(&params).send()?.json()
}

pub fn get_comments(post_id: PostId) -> std::result::Result<Vec<CommentView>, reqwest::Error> {
    let params = GetComments {
        post_id: Some(post_id),
        sort: Some(CommentSortType::Hot),
        type_: Some(ListingType::All),
        ..Default::default()
    };

    let url = format!("{}/comment/list", super::get_api_url());
    let mut comments = CLIENT.get(&url).query(&params).send()?.json::<GetCommentsResponse>()?.comments;

    // hide removed comments
    comments.retain(|c| !c.comment.deleted && !c.comment.removed);

    Ok(comments)
}

pub fn default_post() -> GetPostResponse {
    serde_json::from_str(include_str!("../examples/post.json")).unwrap()
}
