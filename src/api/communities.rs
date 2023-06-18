use lemmy_api_common::{community::{ListCommunities, ListCommunitiesResponse}, lemmy_db_schema::{SortType, SearchType}, lemmy_db_views_actor::structs::CommunityView};

use crate::components::CLIENT;

use super::search;

pub fn fetch_communities(page: i64, query: Option<String>) -> std::result::Result<Vec<CommunityView>, reqwest::Error> {
    if query.is_none() || query.clone().unwrap().trim().is_empty() {
        let params = ListCommunities {
            sort: Some(SortType::TopMonth),
            page: Some(page),
            ..Default::default()
        };

        let url = format!("{}/community/list", super::get_api_url());
        Ok(CLIENT.get(&url).query(&params).send()?.json::<ListCommunitiesResponse>()?.communities)
    } else {
        Ok(search::fetch_search(page, query.unwrap(), Some(SearchType::Communities))?.communities)
    }
}
