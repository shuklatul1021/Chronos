// @generated automatically by Diesel CLI.

diesel::table! {
    job (id) {
        id -> Text,
        job_title -> Text,
        job_description -> Nullable<Text>,
        job_type -> Text,
        payload -> Text,
        runs_at -> Nullable<Text>,
        max_retries -> Int4,
        webhook_url -> Text,
        user_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user (id) {
        id -> Text,
        username -> Text,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(job -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(job, user,);
