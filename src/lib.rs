//! [kimager](https://github.com/nickgerace/kimager) logs the existence of container images on a
//! Kubernetes cluster.

use crate::event_driver::{EventDriver, EventType};
use anyhow::Result;
use futures::{StreamExt, TryStreamExt};
use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, ListParams, WatchEvent},
    Client,
};
use log::{debug, error, warn};

mod event_driver;
mod util;

/// This function is the primary, backend driver for `kimager`.
/// When executed, it watches pods at a cluster-scope and logs the existence of container
/// images on a Kubernetes cluster via the [log](https://crates.io/crates/log) crate.
/// Set the `RUST_LOG` environment variable to change the logging level.
pub async fn watch(client: Client) -> Result<()> {
    let pods: Api<Pod> = Api::all(client.clone());
    let wp = ListParams::default().timeout(0);
    let mut event_driver = EventDriver::new();
    loop {
        debug!("Creating stream with Pods API abstraction...");
        let mut stream = pods.watch(&wp, "0").await?.boxed();
        debug!("Watching events...");
        while let Some(status) = stream.try_next().await? {
            match status {
                WatchEvent::<Pod>::Added(pod) => {
                    event_driver.new_event(pod, EventType::Added).await
                }
                WatchEvent::<Pod>::Deleted(pod) => {
                    event_driver.new_event(pod, EventType::Deleted).await
                }
                WatchEvent::<Pod>::Error(report) => error!("{}", report),
                _ => {}
            }
        }
        warn!("Restarting watcher...");
    }
}
