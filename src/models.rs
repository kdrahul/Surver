use super::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct ERSUsers {
    pub id: i32,
    pub user_group: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub branch: String,
    pub joined_on: chrono::NaiveDateTime,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub organizer: String,
    pub venue: String,
    pub datetime: chrono::NaiveDateTime,
    pub limit: u16,
    pub fee: u32,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Question {
    pub event_id: i32,
    pub question: String,
    pub option1: String,
    pub option2: String,
    pub option3: String,
    pub option4: String,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Survey {
    pub event_id: i32,
    pub user_id: i32,
    pub question_id: i32,
    pub response: String,
    pub date: chrono::NaiveDateTime,
}

// Only used for POST
// Model without ID.
#[derive(Debug, Insertable)]
#[table_name = "ersusers"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub user_group: &'a str,
    pub email: &'a str,
    pub phone: &'a str,
    pub branch: &'a str,
    pub joined_on: chrono::NaiveDateTime,
}
