use diesel::prelude::*;
use diesel::sql_types::*;

table! {
    log (id) {
        id -> Integer,
        created_at -> Timestamptz,
        type -> Text,
        message_json -> Jsonb,
    }
}
