use crate::settings::get_prefs;

pub mod communities;
pub mod community;
pub mod post;
pub mod posts;
pub mod search;
pub mod user;

static API_VERSION: &str = "v3";

pub fn get_api_url() -> String {
    format!("{}/api/{}", get_prefs().instance_url, API_VERSION).to_string()
}
