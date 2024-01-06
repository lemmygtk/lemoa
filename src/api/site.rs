use lemmy_api_common::site::GetSiteResponse;

pub fn fetch_site() -> std::result::Result<GetSiteResponse, reqwest::Error> {
    super::get("/site", &())
}

pub fn default_site_info() -> GetSiteResponse {
    serde_json::from_str(include_str!("../examples/site.json")).unwrap()
}
