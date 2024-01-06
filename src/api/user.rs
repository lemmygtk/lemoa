use lemmy_api_common::{
    lemmy_db_schema::{newtypes::PersonId, CommentSortType},
    person::{
        BlockPerson, BlockPersonResponse, GetPersonDetails, GetPersonDetailsResponse,
        GetPersonMentions, GetPersonMentionsResponse, GetReplies, GetRepliesResponse,
    },
};

pub fn get_user(
    id: PersonId,
    page: i64,
    saved_only: bool,
) -> std::result::Result<GetPersonDetailsResponse, reqwest::Error> {
    let params = GetPersonDetails {
        page: Some(page),
        person_id: Some(id),
        saved_only: Some(saved_only),
        ..Default::default()
    };

    super::get("/user", &params)
}

pub fn block_user(
    person_id: PersonId,
    block: bool,
) -> std::result::Result<BlockPersonResponse, reqwest::Error> {
    let params = BlockPerson { person_id, block };

    super::post("/user/block", &params)
}

pub fn default_person() -> GetPersonDetailsResponse {
    serde_json::from_str(include_str!("../examples/person.json")).unwrap()
}

pub fn get_mentions(
    page: i64,
    unread_only: bool,
) -> std::result::Result<GetPersonMentionsResponse, reqwest::Error> {
    let params = GetPersonMentions {
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
        page: Some(page),
        unread_only: Some(unread_only),
        sort: Some(CommentSortType::New),
        ..Default::default()
    };
    super::get("/user/replies", &params)
}

pub fn mark_all_as_read() -> std::result::Result<GetRepliesResponse, reqwest::Error> {
    super::post("/user/mark_all_as_read", &())
}
