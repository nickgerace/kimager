use anyhow::Result;
use kube::Client;
use log::debug;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::builder().format_module_path(false).init();
    debug!("Starting watcher...");
    kimager::watch(Client::try_default().await?).await?;
    debug!("Watcher has stopped.");
    Ok(())
}
