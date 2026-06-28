use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde_json::Value;

/* #[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::logs)]
pub struct Webhook {
    pub id: i64,
    pub created_at: ,
    pub log_type: String,
pub message: ,
} */

/* CREATE TABLE logs (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    log_type TEXT NOT NULL,
    message JSONB NOT NULL
); */
