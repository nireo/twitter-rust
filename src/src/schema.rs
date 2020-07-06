table! {
    tweet (id) {
        id -> Uuid,
        content -> Text,
        handle -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    user (id) {
        id -> Uuid,
        email -> Text,
        handle -> Text,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    tweet,
    user,
);
