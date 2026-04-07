// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "jobtype"))]
    pub struct Jobtype;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Jobtype;

    job (id) {
        id -> Text,
        jobtitle -> Text,
        jobdescription -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Jobtype,
        payload -> Text,
        runs_at -> Nullable<Text>,
        max_retries -> Int4,
        webhook_url -> Text,
        userId -> Text,
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

diesel::joinable!(job -> user (userId));

diesel::allow_tables_to_appear_in_same_query!(job, user,);
