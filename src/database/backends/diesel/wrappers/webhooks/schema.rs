use diesel::prelude::*;
use diesel::sql_types::*;

table! {
    webhooks (id) {
        id -> Integer,
        discord_webhook_id -> BigInt,
        banned -> Bool,
    }
}
