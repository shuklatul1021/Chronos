use diesel::{RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    connect::Store,
    schema::job,
    structure::job_structure::{Job, JobQueueStructure},
};

impl Store {
    pub fn submit_job(
        &mut self,
        job_title: String,
        job_description: Option<String>,
        job_type: String,
        payload: String,
        runs_at: Option<String>,
        max_retries: i32,
        webhook_url: String,
        user_id: String,
    ) -> Result<bool, diesel::result::Error> {
        let job_uuid = Uuid::new_v4().to_string();
        let temp_title = job_title.clone();
        let job = Job {
            id: job_uuid,
            job_title: job_title,
            job_description: job_description,
            job_type: job_type,
            payload: payload,
            runs_at: runs_at,
            max_retries: max_retries,
            webhook_url: webhook_url,
            user_id: user_id,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };

        let job_result = diesel::insert_into(job::table)
            .values(job)
            .returning(Job::as_returning())
            .get_result(&mut self.connection)
            .expect("Error While Storing Job");
        if job_result.job_title != temp_title {
            return Ok(false);
        }

        let queue_job = JobQueueStructure {
            id: job_result.id.clone(),
            job_title: job_result.job_title.clone(),
            job_description: job_result.job_description.clone(),
            job_type: job_result.job_type.clone(),
            payload: job_result.payload.clone(),
            runs_at: job_result.runs_at.clone(),
            max_retries: job_result.max_retries,
            webhook_url: job_result.webhook_url.clone(),
            user_id: job_result.user_id.clone(),
        };

        let job_value = serde_json::to_string(&queue_job).unwrap();
        let submit_redis_job = self.redis_client.submit_redis_job("", &job_value).unwrap();
        if !submit_redis_job {
            return Ok(false);
        }

        Ok(true)
    }
}
