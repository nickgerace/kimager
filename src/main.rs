/*
 * image-logger
 * https://github.com/nickgerace/image-logger
 * Author: Nick Gerace
 * License: Apache 2.0
 */

use std::env;

use eyre::Result;
use log::info;

#[tokio::main]
async fn main() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    info!(
        "Starting image-logger... (for more information: https://github.com/nickgerace/image-logger)"
    );
    image_logger::run().await?;
    Ok(())
}
