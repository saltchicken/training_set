use anyhow::{Context, Result};

use sqlx::{Pool, Postgres, postgres::PgPoolOptions};


/// Establishes connection to Postgres
pub async fn connect(database_url: &str) -> Result<Pool<Postgres>> {



    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .context("Failed to create connection pool")
}

// This prevents the 'borrowed value does not live long enough' error.
pub fn create_query_builder<'a>(
    classification: Option<&'a str>,
    keywords: Option<&'a str>,
    exclude_keyword: Option<&'a str>,
) -> sqlx::QueryBuilder<'a, Postgres> {
    let mut query_builder =
        sqlx::QueryBuilder::new("SELECT id, image_name, original_image FROM faces WHERE 1=1 ");

    if let Some(cls) = classification {
        query_builder.push(" AND classification = ");
        query_builder.push_bind(cls);
    }

    if let Some(kwd) = keywords {
        query_builder.push(" AND keywords ILIKE "); // ILIKE for case-insensitive
        query_builder.push_bind(format!("%{}%", kwd));
    }

    if let Some(ex_kwd) = exclude_keyword {
        query_builder.push(" AND keywords NOT ILIKE ");
        query_builder.push_bind(format!("%{}%", ex_kwd));
    }

    query_builder
}