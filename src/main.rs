use eyre::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    cluster_image_logger::run().await?;
    Ok(())
}
