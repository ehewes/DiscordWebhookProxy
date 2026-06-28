use super::schema;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde_json::Value;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::webhooks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Webhook {
    pub id: i32,
    pub discord_webhook_id: i64,
    pub banned: bool,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = schema::webhooks)]
pub struct NewWebhook {
    pub discord_webhook_id: i64,
    pub banned: bool,
}

#[derive(AsChangeset, Debug)]
#[diesel(table_name = schema::webhooks)]
pub struct UpdateWebhook {
    pub discord_webhook_id: Option<i64>,
    pub banned: Option<bool>,
}

#[derive(Debug, PartialEq, Eq, FromSqlRow, AsExpression)]
#[diesel(sql_type = "Text")]
pub enum Status {
    Pending,
    Completed,
}
