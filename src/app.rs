pub mod db;
pub mod io;
pub mod models;

use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;
use tracing::{error, info};

/// CLI Arguments structure
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Directory to save images to
    #[arg(short, long)]
    output_dir: PathBuf,

    /// Filter by classification (Exact match)
    #[arg(short, long)]
    classification: Option<String>,

    /// Filter by keywords (Partial match, case-insensitive)
    #[arg(short, long)]
    keywords: Option<String>,

    #[arg(short, long)]
    exclude_keyword: Option<String>,

    /// Database URL (Can be set via env var DB_URL)
    #[arg(long, env = "DB_URL")]
    db_url: String,
}

pub async fn run() -> Result<()> {
    // 1. Parse Arguments
    let args = Args::parse();

    // 2. Validate Input
    io::ensure_directory_exists(&args.output_dir).context("Failed to prepare output directory")?;

    // 3. Connect to Database
    let pool = db::connect(&args.db_url)
        .await
        .context("Failed to connect to database")?;

    info!("Querying database for images...");

    // 4. Create Query Builder (Ownership is moved here)
    let mut query_builder = db::create_query_builder(
        args.classification.as_deref(),
        args.keywords.as_deref(),
        args.exclude_keyword.as_deref(),
    );

    // 5. Build and Stream
    let query = query_builder.build_query_as::<models::ImageRecord>();
    let mut image_stream = query.fetch(&pool);

    // 6. Process Stream
    use futures::StreamExt; // Import extension trait for next()

    let mut count = 0;
    while let Some(result) = image_stream.next().await {
        match result {
            Ok(record) => {
                // Aggressive Extract Method: Logic moved to handle_record
                if let Err(e) = handle_record(&record, &args.output_dir).await {
                    error!("Failed to save image {}: {}", record.id, e);
                } else {
                    count += 1;
                }
            }
            Err(e) => error!("Error fetching record from stream: {}", e),
        }
    }

    info!(
        "Successfully saved {} images to {:?}",
        count, args.output_dir
    );
    Ok(())
}

/// Helper function to handle the saving of a single record
async fn handle_record(record: &models::ImageRecord, output_dir: &PathBuf) -> Result<()> {
    // Determine filename
    let filename = if record.image_name.ends_with(".jpg") || record.image_name.ends_with(".png") {
        record.image_name.clone()
    } else {
        format!("{}.jpg", record.image_name) // Default extension if missing
    };

    let file_path = output_dir.join(filename);

    if let Some(data) = &record.original_image {
        io::write_file(&file_path, data).await?;
    } else {
        tracing::warn!("Skipping image ID {}: No binary data found", record.id);
    }

    Ok(())
}