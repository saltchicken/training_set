mod app;

use anyhow::Result;
use dotenvy::dotenv;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Load environment variables
    dotenv().ok();

    // 2. Initialize logging
    tracing_subscriber::fmt::init();

    info!("Starting Image Extractor...");

    // 3. Delegate execution to the app module
    app::run().await?;

    info!("Extraction complete.");
    Ok(())
}
