use diesel::prelude::{Insertable, Queryable, Selectable};
use serde::Serialize;

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::job)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Job {
    pub id: String,
    pub job_title: String,
    pub job_description: Option<String>,
    pub job_type: String,
    pub payload: String,
    pub runs_at: Option<String>,
    pub max_retries: i32,
    pub webhook_url: String,
    pub user_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize)]
pub struct JobQueueStructure {
    pub id: String,
    pub job_title: String,
    pub job_description: Option<String>,
    pub job_type: String,
    pub payload: String,
    pub runs_at: Option<String>,
    pub max_retries: i32,
    pub webhook_url: String,
    pub user_id: String,
}
