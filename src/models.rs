use super::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Events {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub venue: Option<String>,
    pub starts_at: Option<chrono::NaiveDateTime>,
    pub max_limit: Option<i32>,
    pub fee: Option<i32>,
    pub prize_money: Option<i32>,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Users {
    pub id: i32,
    pub username: Option<String>,
    pub password: Option<String>,

    pub role: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub branch: Option<String>,
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
    pub answer: Option<String>,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct SurveyResponse {
    pub id: i32,
    pub event_id: i32,
    pub user_id: i32,
    pub question_id: i32,
    pub user_response: Option<String>,
    pub date: chrono::NaiveDateTime,
}

// Only used for POST
// Model without ID.
#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub role: &'a str,
    pub email: &'a str,
    pub phone: &'a str,
    pub branch: &'a str,
}

#[derive(Debug, Insertable)]
#[table_name = "events"]
pub struct NewEvent<'a> {
    pub name: &'a str,
    pub venue: &'a str,
    pub description: &'a str,
    pub starts_at: chrono::NaiveDateTime,
    pub max_limit: &'a i32,
    pub fee: &'a i32,
    pub prize_money: &'a i32,
}

#[derive(Debug, Insertable)]
#[table_name = "questions"]
pub struct NewQuestion<'a> {
    pub question: &'a str,
    pub option1: &'a str,
    pub option2: &'a str,
    pub option3: &'a str,
    pub option4: &'a str,
    pub answer: &'a str,
    pub event_id: &'a i32,
}
