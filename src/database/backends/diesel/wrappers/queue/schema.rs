use diesel::prelude::*;
use diesel::sql_types::*;

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
