use lemmy_api_common::site::{GetFederatedInstancesResponse, GetFederatedInstances, FederatedInstances};
use reqwest::Error;
use crate::settings;


pub fn fetch_instances() ->  Option<FederatedInstances> {
    // TODO: Update code to use the Instance views from lemmy 0.18.0 
    let params = GetFederatedInstances {
        auth: settings::get_current_account().jwt,
    };
     let instances: std::result::Result<GetFederatedInstancesResponse, reqwest::Error>  = super::get("/federated_instances", &params);

    
    match instances
    {
        Ok(instances) =>
        {
               instances.federated_instances
        }
        Err(e) => 
        {
            None
        }
    }


}

