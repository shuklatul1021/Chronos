
pub enum JobStatus { Pending , Completed, Failed }
pub struct Job {
    pub id: String,
    pub job_type: String,
    pub result : String,
    pub status: JobStatus,
    pub payload: String,
    pub duration_ms : i32,
    pub attempts : i32
}
