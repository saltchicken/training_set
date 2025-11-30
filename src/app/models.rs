// ‼️ NEW FILE: Definitions based on schema
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct ImageRecord {
    pub id: i32,
    pub image_name: String,
    // ‼️ Mapping 'bytea' to Vec<u8>. Option because schema says Nullable.
    pub original_image: Option<Vec<u8>>,
}
