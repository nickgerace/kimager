mod event_driver;
mod util;
mod watcher;

use eyre::Result;
use kube::Client;
use log::debug;

/// This function is the primary, backend driver for `cluster-image-logger`.
/// When executed, results will be logged via the [log](https://crates.io/crates/log) crate.
/// Set the `RUST_LOG` environment variable to change the logging level.
pub async fn run() -> Result<()> {
    env_logger::builder().format_module_path(false).init();
    debug!("Creating Kubernetes client...");
    let client = Client::try_default().await?;
    watcher::watcher(client.clone()).await?;
    debug!("Watcher has stopped.");
    Ok(())
}
