use lemmy_api_common::{person::{GetPersonDetailsResponse, GetPersonDetails}};
use crate::components::CLIENT;

pub fn get_user(username: String, page: i64) -> std::result::Result<GetPersonDetailsResponse, reqwest::Error> {
    let params = GetPersonDetails {
        page: Some(page),
        username: Some(username),
        ..Default::default()
    };

    let url = format!("{}/user", super::get_api_url());
    CLIENT.get(&url).query(&params).send()?.json()
}

pub fn default_person() -> GetPersonDetailsResponse {
    serde_json::from_str(include_str!("../examples/person.json")).unwrap()
}