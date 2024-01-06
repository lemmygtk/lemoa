use lemmy_api_common::site::{GetFederatedInstancesResponse, InstanceWithFederationState};

pub fn fetch_instances(
    query_filter: &str,
) -> std::result::Result<Vec<InstanceWithFederationState>, reqwest::Error> {
    // we fetch the instances from the official instance because the instance is likely unset on first startup
    let instances = super::CLIENT
        .get("https://lemmy.ml/api/v3/federated_instances".to_owned())
        .send()?
        .json::<GetFederatedInstancesResponse>()?;

    let lowercase_query_filter = query_filter.to_lowercase();
    match instances.federated_instances {
        Some(instances) => Ok(instances
            .linked
            .iter()
            .filter(|instance| {
                instance.instance.software == Some("lemmy".to_owned())
                    && instance
                        .instance
                        .domain
                        .clone()
                        .contains(&lowercase_query_filter)
            })
            .cloned()
            .collect::<Vec<InstanceWithFederationState>>()),
        None => Ok(vec![]),
    }
}
