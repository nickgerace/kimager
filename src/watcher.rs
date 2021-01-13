/*
 * cluster-image-logger
 * https://github.com/nickgerace/cluster-image-logger
 * Author: Nick Gerace
 * License: Apache 2.0
 */

use crate::event_driver::{EventDriver, EventType};

use eyre::Result;
use futures::{StreamExt, TryStreamExt};
use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, ListParams, WatchEvent},
    Client,
};
use log::{debug, error, warn};

/// Watch pods on cluster scope and log the results via the [log](https://crates.io/crates/log) crate.
pub async fn watcher(client: Client) -> Result<()> {
    debug!("Creating Pods API abstraction and event driver...");
    let pods: Api<Pod> = Api::all(client.clone());
    let wp = ListParams::default().timeout(0);
    let mut event_driver = EventDriver::new();
    loop {
        debug!("Creating stream with Pods API abstraction..");
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
