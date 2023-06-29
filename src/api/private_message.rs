use lemmy_api_common::{
    lemmy_db_schema::newtypes::{PersonId, PrivateMessageId},
    private_message::{
        CreatePrivateMessage, EditPrivateMessage, GetPrivateMessages, PrivateMessageResponse,
        PrivateMessagesResponse,
    },
};

pub fn create_private_message(
    content: String,
    recipient_id: PersonId,
) -> std::result::Result<PrivateMessageResponse, reqwest::Error> {
    let params = CreatePrivateMessage {
        auth: crate::settings::get_current_account().jwt.unwrap(),
        recipient_id,
        content,
    };
    super::post("/private_message", &params)
}

pub fn edit_private_message(
    content: String,
    private_message_id: PrivateMessageId,
) -> std::result::Result<PrivateMessageResponse, reqwest::Error> {
    let params = EditPrivateMessage {
        auth: crate::settings::get_current_account().jwt.unwrap(),
        private_message_id,
        content,
    };
    super::put("/private_message", &params)
}

pub fn list_private_messages(
    unread_only: bool,
    page: i64,
) -> std::result::Result<PrivateMessagesResponse, reqwest::Error> {
    let params = GetPrivateMessages {
        unread_only: Some(unread_only),
        page: Some(page),
        auth: crate::settings::get_current_account().jwt.unwrap(),
        ..Default::default()
    };
    super::get("/private_message/list", &params)
}
