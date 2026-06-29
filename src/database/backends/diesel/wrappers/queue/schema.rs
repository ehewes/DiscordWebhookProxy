diesel::table! {
    queue (id) {
        id -> Integer,
        webhook_id -> Integer,
        created_at -> Timestamp,
        message -> Jsonb,
        status -> Text,
        updated_at -> Timestamp,
    }
}
