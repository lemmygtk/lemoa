use crate::settings;
use lemmy_api_common::{
    lemmy_db_schema::source::instance::Instance,
    site::{GetFederatedInstances, GetFederatedInstancesResponse},
};

pub fn fetch_instances(query_filter: &str) -> std::result::Result<Vec<Instance>, reqwest::Error> {
    // TODO: Update code to use the Instance views from lemmy 0.18.0
    let params = GetFederatedInstances {
        auth: settings::get_current_account().jwt,
    };

    // we fetch the instances from the official instance because the instance is likely unset on first startup
    let instances = super::CLIENT
        .get("https://lemmy.ml/api/v3/federated_instances".to_owned())
        .query(&params)
        .send()?
        .json::<GetFederatedInstancesResponse>()?;

    let lowercase_query_filter = query_filter.to_lowercase();
    match instances.federated_instances {
        Some(instances) => Ok(instances
            .linked
            .iter()
            .filter(|instance| instance.software == Some("lemmy".to_owned()) && instance.domain.clone().contains(&lowercase_query_filter))
            .map(|instance| instance.clone())
            .collect::<Vec<Instance>>()),
        None => Ok(vec![]),
    }
}
