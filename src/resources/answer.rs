use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Deserialize)]
pub struct Answer {
    pub id: i32,
    pub body: String,
    pub isCorrect: bool,
    pub questionId: i32,
}
