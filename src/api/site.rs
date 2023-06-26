use lemmy_api_common::site::{GetSite, GetSiteResponse};

use crate::settings;

pub fn fetch_site() -> std::result::Result<GetSiteResponse, reqwest::Error> {
    let params = GetSite {
        auth: settings::get_current_account().jwt,
    };
    super::get("/site", &params)
}
