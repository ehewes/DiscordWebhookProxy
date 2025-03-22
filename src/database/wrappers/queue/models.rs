use super::schema;
use diesel::prelude::*;
use serde_json::Value;
use chrono::{DateTime, Utc};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::webhooks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Webhook {
    pub id: i32,
    pub discord_webhook_id: i64,
    pub banned: bool,
}