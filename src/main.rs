/*
 * cluster-image-logger
 * https://github.com/nickgerace/cluster-image-logger
 * Author: Nick Gerace
 * License: Apache 2.0
 */

use std::env;

use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    cluster_image_logger::run().await?;
    Ok(())
}
