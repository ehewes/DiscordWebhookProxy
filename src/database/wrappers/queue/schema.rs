#[macro_use]
extern crate diesel;

pub mod schema {
    use diesel::prelude::*;
    use diesel::sql_types::*;

    table! {
        webhooks (id) {
            id -> Integer,
            discord_webhook_id -> BigInt,
            banned -> Bool,
        }
    }

    table! {
        queue (id) {
            id -> Integer,
            webhook_id -> Integer,
            created_at -> Timestamptz,
            message -> Jsonb,
            status -> Text, 
            updated_at -> Timestamptz,
        }
    }
}