use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAuthStructure {
    pub id : String,
    pub email : String,
    pub exp: usize,
}