use serde::{de::DeserializeOwned, Serialize};

use crate::{
    config,
    settings::{self, get_current_account},
};

pub mod auth;
pub mod comment;
pub mod communities;
pub mod community;
pub mod image;
pub mod instances;
pub mod moderation;
pub mod post;
pub mod posts;
pub mod private_message;
pub mod search;
pub mod site;
pub mod user;

static API_VERSION: &str = "v3";

use relm4::once_cell::sync::Lazy;
use reqwest::{blocking::Client, header::HeaderMap, header::HeaderValue};

pub static CLIENT: Lazy<Client> = Lazy::new(|| {
    let user_agent = format!("{}/{}", config::NAME, config::VERSION);
    Client::builder()
        .user_agent(user_agent)
        .build()
        .expect("Failed to create reqwest http client!")
});

fn get_api_url() -> String {
    format!("{}/api/{}", get_current_account().instance_url, API_VERSION)
}

fn get_url(path: &str) -> String {
    format!("{}{}", get_api_url(), path)
}

fn get_auth_header() -> HeaderMap<HeaderValue> {
    let mut headers = HeaderMap::new();

    if let Some(jwt) = settings::get_current_account().jwt {
        let auth_string = "Bearer ".to_string() + &jwt.into_inner();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&auth_string).unwrap(),
        );
    }

    headers
}

fn get<T, Params>(path: &str, params: &Params) -> Result<T, reqwest::Error>
where
    T: DeserializeOwned,
    Params: Serialize + std::fmt::Debug,
{
    CLIENT
        .get(get_url(path))
        .headers(get_auth_header())
        .query(&params)
        .send()?
        .json()
}

fn post<T, Params>(path: &str, params: &Params) -> Result<T, reqwest::Error>
where
    T: DeserializeOwned,
    Params: Serialize + std::fmt::Debug,
{
    CLIENT
        .post(get_url(path))
        .headers(get_auth_header())
        .json(&params)
        .send()?
        .json()
}

fn put<T, Params>(path: &str, params: &Params) -> Result<T, reqwest::Error>
where
    T: DeserializeOwned,
    Params: Serialize + std::fmt::Debug,
{
    CLIENT
        .put(get_url(path))
        .headers(get_auth_header())
        .json(&params)
        .send()?
        .json()
}
