/* use super::schema;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::webhooks)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Pending,
    Completed,
} */
