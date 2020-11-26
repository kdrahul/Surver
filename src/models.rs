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
    pub description: Option<String>,
    pub organizer: Option<String>,
    //pub venue: Option<String>,
    pub datetime: Option<chrono::NaiveDateTime>,
    pub limit: Option<i16>,
    pub fee: Option<i32>,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Question {
    pub event_id: i32,
    pub id: i32,
    pub question: Option<String>,
    pub option1: Option<String>,
    pub option2: Option<String>,
    pub option3: Option<String>,
    pub option4: Option<String>,
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

#[derive(Debug, Insertable)]
#[table_name = "event"]
pub struct NewEvent<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub organizers: &'a str,
    pub starts_at: chrono::NaiveDateTime,
    pub max_participants: &'a i16,
    pub fee: &'a i32,
}
