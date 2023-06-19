use lemmy_api_common::community::{GetCommunity, GetCommunityResponse};

pub fn get_community(name: String) -> std::result::Result<GetCommunityResponse, reqwest::Error> {
    let params = GetCommunity {
        name: Some(name),
        ..Default::default()
    };

    super::get("/community", &params)
}

pub fn default_community() -> GetCommunityResponse {
    serde_json::from_str(include_str!("../examples/community.json")).unwrap()
}
