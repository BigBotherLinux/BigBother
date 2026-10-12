//! Where the IP address says the user is. The same lookup Anaconda uses to
//! preselect a timezone.

use bevy::tasks::{IoTaskPool, Task};
use serde::Deserialize;

const URL: &str = "https://geoip.fedoraproject.org/city";

#[derive(Deserialize, Clone, Debug)]
pub struct Estimate {
    pub latitude: f64,
    pub longitude: f64,
    pub city: Option<String>,
    pub time_zone: Option<String>,
}

pub fn lookup(agent: ureq::Agent) -> Task<Option<Estimate>> {
    IoTaskPool::get().spawn(async move {
        agent
            .get(URL)
            .call()
            .and_then(|mut response| response.body_mut().read_json::<Estimate>())
            .inspect_err(|e| bevy::log::warn!("GeoIP lookup failed: {e}"))
            .ok()
    })
}
