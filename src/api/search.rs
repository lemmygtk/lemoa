use lemmy_api_common::{site::{SearchResponse, Search}, lemmy_db_schema::{SortType, SearchType}};

use crate::util;

pub fn fetch_search(page: i64, query: String, search_type: Option<SearchType>) -> std::result::Result<SearchResponse, reqwest::Error> {
    let params = Search {
        q: query,
        sort: Some(SortType::TopMonth),
        page: Some(page),
        type_: search_type,
        auth: util::get_auth_token(),
        ..Default::default()
    };

    super::get("/search", &params)
}