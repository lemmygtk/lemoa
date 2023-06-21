use lemmy_api_common::{person::{GetPersonDetailsResponse, GetPersonDetails}};

use crate::settings;

pub fn get_user(username: String, page: i64) -> std::result::Result<GetPersonDetailsResponse, reqwest::Error> {
    let params = GetPersonDetails {
        page: Some(page),
        username: Some(username),
        auth: settings::get_current_account().jwt,
        ..Default::default()
    };

    super::get("/user", &params)
}

pub fn default_person() -> GetPersonDetailsResponse {
    serde_json::from_str(include_str!("../examples/person.json")).unwrap()
}