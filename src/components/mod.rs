pub mod post_row;
pub mod community_row;
pub mod profile_page;
pub mod community_page;
pub mod post_page;
pub mod comment_row;

use reqwest::blocking::Client;
use relm4::once_cell::sync::Lazy;

pub static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::new()
});