use lemmy_api_common::{post::{GetPostsResponse, GetPosts}, lemmy_db_views::structs::PostView};

use crate::components::CLIENT;

pub fn list_posts(page: i64, community_name: Option<String>) -> std::result::Result<Vec<PostView>, reqwest::Error> {
    let params = GetPosts {
        page: Some(page),
        community_name,
        ..Default::default()
    };

    let url = format!("{}/post/list", super::get_api_url());
    Ok(CLIENT.get(&url).query(&params).send()?.json::<GetPostsResponse>()?.posts)
}
