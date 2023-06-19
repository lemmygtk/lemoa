use serde::{de::DeserializeOwned, Serialize};

use crate::settings::get_prefs;

pub mod communities;
pub mod community;
pub mod post;
pub mod posts;
pub mod search;
pub mod user;
pub mod auth;

static API_VERSION: &str = "v3";

use reqwest::blocking::Client;
use relm4::once_cell::sync::Lazy;

pub static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::new()
});

fn get_api_url() -> String {
    format!("{}/api/{}", get_prefs().instance_url, API_VERSION).to_string()
}

fn get_url(path: &str) -> String {
    format!("{}{}", get_api_url(), path)
}

fn get<T, Params>(path: &str, params: &Params) -> Result<T, reqwest::Error>
where
    T: DeserializeOwned,
    Params: Serialize + std::fmt::Debug,
{
    CLIENT
        .get(&get_url(path))
        .query(&params)
        .send()?
        .json()
}
