use lemmy_api_common::{
    community::{ListCommunities, ListCommunitiesResponse},
    lemmy_db_schema::{ListingType, SearchType, SortType},
    lemmy_db_views_actor::structs::CommunityView,
};

use super::search;
use crate::settings;

pub fn fetch_communities(
    page: i64,
    query: Option<String>,
    listing_type: Option<ListingType>,
) -> std::result::Result<Vec<CommunityView>, reqwest::Error> {
    if query.is_none() || query.clone().unwrap().trim().is_empty() {
        let params = ListCommunities {
            type_: listing_type,
            sort: Some(SortType::TopMonth),
            page: Some(page),
            auth: settings::get_current_account().jwt,
            ..Default::default()
        };

        Ok(super::get::<ListCommunitiesResponse, _>("/community/list", &params)?.communities)
    } else {
        Ok(search::fetch_search(page, query.unwrap(), Some(SearchType::Communities))?.communities)
    }
}
