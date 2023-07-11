use lemmy_api_common::{
    lemmy_db_schema::{newtypes::PersonId, CommentSortType},
    person::{
        GetPersonDetails, GetPersonDetailsResponse, GetPersonMentions, GetPersonMentionsResponse,
        GetReplies, GetRepliesResponse, MarkAllAsRead,
    },
};

use crate::settings;

pub fn get_user(
    id: PersonId,
    page: i64,
    saved_only: bool,
) -> std::result::Result<GetPersonDetailsResponse, reqwest::Error> {
    let params = GetPersonDetails {
        page: Some(page),
        person_id: Some(id),
        auth: settings::get_current_account().jwt,
        saved_only: Some(saved_only),
        ..Default::default()
    };

    super::get("/user", &params)
}

pub fn default_person() -> GetPersonDetailsResponse {
    serde_json::from_str(include_str!("../examples/person.json")).unwrap()
}

pub fn get_mentions(
    page: i64,
    unread_only: bool,
) -> std::result::Result<GetPersonMentionsResponse, reqwest::Error> {
    let params = GetPersonMentions {
        auth: settings::get_current_account().jwt.unwrap(),
        unread_only: Some(unread_only),
        page: Some(page),
        sort: Some(CommentSortType::New),
        ..Default::default()
    };
    super::get("/user/mention", &params)
}

pub fn get_replies(
    page: i64,
    unread_only: bool,
) -> std::result::Result<GetRepliesResponse, reqwest::Error> {
    let params = GetReplies {
        auth: settings::get_current_account().jwt.unwrap(),
        page: Some(page),
        unread_only: Some(unread_only),
        sort: Some(CommentSortType::New),
        ..Default::default()
    };
    super::get("/user/replies", &params)
}

pub fn mark_all_as_read() -> std::result::Result<GetRepliesResponse, reqwest::Error> {
    let params = MarkAllAsRead {
        auth: settings::get_current_account().jwt.unwrap(),
    };
    super::post("/user/mark_all_as_read", &params)
}
