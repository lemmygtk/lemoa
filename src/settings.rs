use std::{fs::File, path::PathBuf};
use crate::gtk::glib;
use lemmy_api_common::sensitive::Sensitive;
use serde::{Deserialize, Serialize};
use crate::APP_ID;

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct Account {
    pub instance_url: String,
    pub jwt: Option<Sensitive<String>>,
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize, Serialize, Default)]
pub struct Preferences {
    pub accounts: Vec<Account>,
    pub current_account_index: u32
}

pub fn data_path() -> PathBuf {
    let mut path = glib::user_data_dir();
    path.push(APP_ID);
    std::fs::create_dir_all(&path).expect("Could not create directory.");
    path.push("data.json");
    path
}

pub fn save_prefs(prefs: &Preferences) {
    let file = File::create(data_path()).expect("Could not create json file.");
    serde_json::to_writer(file, &prefs).expect("Could not write data to json file");
}

pub fn get_prefs() -> Preferences {
    if let Ok(file) = File::open(data_path()) {
        // Deserialize data from file to vector
        let prefs: Result<Preferences, serde_json::Error> = serde_json::from_reader(file);
        if prefs.is_ok() {
            return prefs.unwrap();
        }
    }
    return Preferences::default();
}

pub fn get_current_account() -> Account {
    let mut prefs = get_prefs();
    if prefs.accounts.len() == 0 {
        prefs.accounts.push(Account::default());
        save_prefs(&prefs);
    }
    prefs.accounts[prefs.current_account_index as usize].clone()
}

pub fn update_current_account(account: Account) {
    let mut settings = get_prefs();
    settings.accounts[settings.current_account_index as usize] = account;
    save_prefs(&settings);
}
