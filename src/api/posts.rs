use lemmy_api_common::{post::{GetPostsResponse, GetPosts}, lemmy_db_views::structs::PostView};

pub fn list_posts(page: i64, community_name: Option<String>) -> std::result::Result<Vec<PostView>, reqwest::Error> {
    let params = GetPosts {
        page: Some(page),
        community_name,
        ..Default::default()
    };

    Ok(super::get::<GetPostsResponse, _>("/post/list", &params)?.posts)
}
