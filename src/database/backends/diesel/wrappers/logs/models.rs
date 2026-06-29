/* use super::schema;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde_json::Value;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::logs)]
pub struct Log {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub r#type: String,
    pub message_json: Value,
} */
