use crate::settings;
use itertools::Itertools;
use lemmy_api_common::{
    comment::{GetComments, GetCommentsResponse},
    lemmy_db_schema::{
        newtypes::{CommunityId, PostId},
        CommentSortType, ListingType,
    },
    lemmy_db_views::structs::CommentView,
    post::{
        CreatePost, CreatePostLike, DeletePost, EditPost, GetPost, GetPostResponse, PostResponse,
        SavePost,
    },
};
use std::result::Result;

pub fn get_post(id: PostId) -> Result<GetPostResponse, reqwest::Error> {
    let params = GetPost {
        id: Some(id),
        comment_id: None,
        auth: settings::get_current_account().jwt,
    };

    super::get("/post", &params)
}

pub fn get_comments(post_id: PostId) -> Result<Vec<CommentView>, reqwest::Error> {
    let params = GetComments {
        post_id: Some(post_id),
        sort: Some(CommentSortType::Hot),
        type_: Some(ListingType::All),
        max_depth: Some(8),
        auth: settings::get_current_account().jwt,
        ..Default::default()
    };

    let mut comments = super::get::<GetCommentsResponse, _>("/comment/list", &params)?.comments;

    // hide removed and deleted comments
    comments.retain(|c| !c.comment.deleted && !c.comment.removed);

    // group comments by their parent and generate the tree structure known from the web interface
    let mut grouped_comments: Vec<CommentView> = vec![];
    for (_, comments_group) in &comments
        .iter()
        .group_by(|a| a.comment.path.split(".").collect::<Vec<&str>>()[1].to_owned())
    {
        let mut group = comments_group.collect::<Vec<&CommentView>>();
        group.sort_by(|a, b| a.comment.path.partial_cmp(&b.comment.path).unwrap());
        for c in group {
            grouped_comments.push(c.clone());
        }
    }

    Ok(grouped_comments)
}

pub fn default_post() -> GetPostResponse {
    serde_json::from_str(include_str!("../examples/post.json")).unwrap()
}

pub fn create_post(
    name: String,
    body: String,
    url: Option<reqwest::Url>,
    community_id: i32,
) -> Result<PostResponse, reqwest::Error> {
    let params = CreatePost {
        name,
        body: Some(body),
        url,
        community_id: CommunityId(community_id),
        auth: settings::get_current_account().jwt.unwrap(),
        ..Default::default()
    };
    super::post("/post", &params)
}

pub fn edit_post(
    name: String,
    url: Option<reqwest::Url>,
    body: String,
    post_id: i32,
) -> Result<PostResponse, reqwest::Error> {
    let params = EditPost {
        name: Some(name),
        body: Some(body),
        url,
        post_id: PostId(post_id),
        auth: settings::get_current_account().jwt.unwrap(),
        ..Default::default()
    };
    super::put("/post", &params)
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

pub fn save_post(post_id: PostId, save: bool) -> Result<PostResponse, reqwest::Error> {
    let params = SavePost {
        auth: settings::get_current_account().jwt.unwrap(),
        post_id,
        save,
    };
    super::put("/post/save", &params)
}
