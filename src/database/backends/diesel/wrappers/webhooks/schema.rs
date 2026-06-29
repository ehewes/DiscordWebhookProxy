diesel::table! {
    webhooks (id) {
        id -> Integer,
        discord_webhook_id -> BigInt,
        banned -> Bool,
    }
}
