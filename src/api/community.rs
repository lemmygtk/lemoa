use lemmy_api_common::community::{GetCommunity, GetCommunityResponse};

use crate::components::CLIENT;

pub fn get_community(name: String) -> std::result::Result<GetCommunityResponse, reqwest::Error> {
    let params = GetCommunity {
        name: Some(name),
        ..Default::default()
    };

    let url = format!("{}/community", super::get_api_url());
    CLIENT.get(&url).query(&params).send()?.json()
}

pub fn default_community() -> GetCommunityResponse {
    serde_json::from_str(include_str!("../examples/community.json")).unwrap()
}
