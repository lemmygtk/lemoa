use lemmy_api_common::site::{GetSiteResponse, GetSite, FederatedInstances};
use reqwest::Error;

use crate::settings;

pub fn fetch_site() -> std::result::Result<GetSiteResponse, reqwest::Error> {
    let params = GetSite {
        auth: settings::get_current_account().jwt,
    };
    super::get("/site", &params)
}

pub fn fetch_instances() {
    // TODO: Update code to use the Instance views from lemmy 0.18.0 
    let params = GetSite {
        auth: Option::None,
    };
     let site: std::result::Result<GetSiteResponse, reqwest::Error>  = super::get("/site", &params);
    // match site
    // {
    //     Ok(site) =>
    //         {
    //             site.federated_instances
    //         }
    //     Err(_) => {
    //         None
    //     }
    // }


}

