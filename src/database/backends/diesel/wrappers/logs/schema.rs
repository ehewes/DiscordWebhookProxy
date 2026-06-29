diesel::table! {
    logs (id) {
        id -> Integer,
        created_at -> Timestamp,
        r#type -> Text,
        message_json -> Jsonb,
    }
}
