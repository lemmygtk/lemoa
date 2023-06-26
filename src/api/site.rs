use lemmy_api_common::site::{GetSiteResponse, GetSite, FederatedInstances};
use reqwest::Error;

use crate::settings;

pub fn fetch_site() -> std::result::Result<GetSiteResponse, reqwest::Error> {
    let params = GetSite {
        auth: settings::get_current_account().jwt,
    };
    super::get("/site", &params)
}
