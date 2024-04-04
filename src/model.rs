use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, Clone, FromRow)]
pub struct UrlModel {
    pub url_id: String,
    pub org_url: String,
    pub short_url: String,
    pub cust_phrase: String,
    pub inserted_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub visits: i16,
    pub url_code: i32,
}

impl UrlModel {
    pub fn empty() -> Self {
        UrlModel {
            url_id: "".to_string(),
            org_url: "".to_string(),
            short_url: "".to_string(),
            cust_phrase: "".to_string(),
            inserted_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            visits: 0,
            url_code: 0,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct InputSingleText {
    pub input_string: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateUrl {
    pub org_url: String,
    pub cust_phrase: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InputKey {
    pub input_key: String,
}
