use lemmy_api_common::lemmy_db_schema::newtypes::DbUrl;
use relm4_components::web_image::WebImageMsg;

pub fn get_web_image_msg(url: Option<DbUrl>) -> WebImageMsg {
    if let Some(url) = url {
        WebImageMsg::LoadImage(url.to_string())
    } else {
        WebImageMsg::Unload
    }
}

pub fn get_web_image_url(url: Option<DbUrl>) -> String {
    if let Some(url) = url {
        url.to_string()
    } else {
        "".to_string()
    }
}

pub fn markdown_to_pango_markup(text: String) -> String {
    html2pango::markup_html(&markdown::to_html(&text)).unwrap_or(text)
}

pub fn format_elapsed_time(time: chrono::NaiveDateTime) -> String {
    let formatter = timeago::Formatter::new();
    let current_time = chrono::Utc::now();
    let published = time.and_utc();
    formatter.convert_chrono(published, current_time)
}
