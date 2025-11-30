
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct ImageRecord {
    pub id: i32,
    pub image_name: String,

    pub original_image: Option<Vec<u8>>,
}