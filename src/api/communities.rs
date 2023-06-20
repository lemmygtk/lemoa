use lemmy_api_common::{community::{ListCommunities, ListCommunitiesResponse}, lemmy_db_schema::{SortType, SearchType}, lemmy_db_views_actor::structs::CommunityView};

use crate::util;

use super::search;

pub fn fetch_communities(page: i64, query: Option<String>) -> std::result::Result<Vec<CommunityView>, reqwest::Error> {
    if query.is_none() || query.clone().unwrap().trim().is_empty() {
        let params = ListCommunities {
            sort: Some(SortType::TopMonth),
            page: Some(page),
            auth: util::get_auth_token(),
            ..Default::default()
        };

        Ok(super::get::<ListCommunitiesResponse, _>("/community/list", &params)?.communities)
    } else {
        Ok(search::fetch_search(page, query.unwrap(), Some(SearchType::Communities))?.communities)
    }
}
