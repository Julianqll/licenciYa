use serde::{Deserialize, Serialize};
use serde_json;

use super::answer::Answer;

#[derive(Debug, Deserialize, Clone)]
pub struct Question {
    pub id: i32,
    pub title: String,
    pub image: Option<String>,
    pub answerList: Vec<Answer>
}
