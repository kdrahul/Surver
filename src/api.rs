use super::models::{Users, NewUser, Events, NewEvent, Question, NewQuestion, Responses,NewResponse};
use diesel::dsl::count_star;
use super::schema::users::dsl::*;
use super::schema::events::dsl::*;
use super::schema::questions::dsl::*;
use super::schema::response::dsl::*;
use actix_web::{web, HttpResponse, Error};
use super::Pool;
use serde::{Serialize, Deserialize};
use diesel::dsl::{insert_into, delete};
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use std::vec::Vec;


#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub role: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub email: String,
    pub phone: String,
    pub branch: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EventData {
    pub name: String,
    pub description: String,
    pub max_limit: i32,
    pub fee: i32,
    pub prize_money: i32,
    pub venue: String,
    pub starts_at: String,
}



// API for /users
pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_users(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn get_users_by_id(
    db: web::Data<Pool>,
    userid: web::Path<i32>
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || user_by_id(db, userid.into_inner()))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?
    )
} 


pub async fn add_users(
    db: web::Data<Pool>,
    item: web::Json<UserData>
    ) -> Result<HttpResponse, Error> {
    Ok(web::block(move || single_user(db, item))
        .await
        .map(|user| HttpResponse::Created().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
    
} 
pub async fn delete_users(
    db: web::Data<Pool>,
    userid: web::Path<i32>
    ) -> Result<HttpResponse, Error> {
    Ok(web::block(move || del_user(db, userid.into_inner()))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?
        )
}

pub async fn get_events(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_events(db))
        .await
        .map(|survey| HttpResponse::Ok().json(survey))
        .map_err(|_| HttpResponse::InternalServerError())?
        )
}

pub async fn get_event_by_id(db: web::Data<Pool>, eventid: web::Path<i32>) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || get_single_event(db, eventid.into_inner()))
        .await
        .map(|ev| HttpResponse::Ok().json(ev))
        .map_err(|_| HttpResponse::InternalServerError())?
    )
}
pub async fn add_event(
    db: web::Data<Pool>,
    item: web::Json<EventData>
    ) -> Result<HttpResponse, Error> {
    Ok(web::block(move || add_single_event(db, item))
        .await
        .map(|ev| HttpResponse::Created().json(ev))
        .map_err(|_| HttpResponse::InternalServerError())?)
} 

pub async fn delete_event(
    db: web::Data<Pool>,
    eventid: web::Path<i32>
    ) -> Result<HttpResponse, Error> {
    Ok(web::block(move || delete_single_event(db, eventid.into_inner()))
        .await
        .map(|eve| HttpResponse::Ok().json(eve))
        .map_err(|_| HttpResponse::InternalServerError())?
        )
}

pub async fn get_event_count(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_count(db))
        .await
        .map(|data| HttpResponse::Ok().json(data))
        .map_err(|_| HttpResponse::InternalServerError())?
    )
}
fn get_count(db: web::Data<Pool>) -> Result<i64, diesel::result::Error> {
    let conn = db.get().unwrap();
    let res = events.select(count_star()).first::<i64>(&conn);
    Ok(res.unwrap())
}

fn get_single_event(db: web::Data<Pool>, eventid: i32) -> Result<Events, diesel::result::Error> {
    let conn = db.get().unwrap();
    let res = events.find(eventid).get_result::<Events>(&conn)?;
    Ok(res)
}

fn get_all_events(db: web::Data<Pool>) -> Result<Vec<Events>, diesel::result::Error>{
    let conn = db.get().unwrap();
    let res = events.load::<Events>(&conn)?;
    Ok(res)
}

fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<Users>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = users.load::<Users>(&conn)?;
    Ok(items)
}

fn user_by_id(pool: web::Data<Pool>, userid: i32) -> Result<Users, diesel::result::Error> {
    let conn = pool.get().unwrap();
    users.find(userid).get_result::<Users>(&conn)
}

fn single_user(
    db: web::Data<Pool>,
    userdata: web::Json<UserData> 
    ) -> Result<Users, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_user = NewUser {
        username: &userdata.username,
        password: &userdata.password,
        first_name: &userdata.first_name,
        last_name: &userdata.last_name,

        role: &userdata.role,
        email: &userdata.email,
        phone:  &userdata.phone,
        branch: &userdata.branch,
    };
    let res = insert_into(users).values(&new_user).get_result(&conn)?;
    Ok(res)
}

fn add_single_event(
    db: web::Data<Pool>,
    eventdata: web::Json<EventData> 
    ) -> Result<Events, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_event = NewEvent {
        name: &eventdata.name,
        description: &eventdata.description,
        starts_at: chrono::NaiveDateTime::parse_from_str(&eventdata.starts_at, "%Y-%m-%d %H:%M:%S").unwrap(),
        max_limit: &eventdata.max_limit,
        fee: &eventdata.fee,
        prize_money: &eventdata.prize_money,
        venue: &eventdata.venue,
    };
    dbg!(&new_event);
    let res = insert_into(events).values(&new_event).get_result(&conn)?;
    Ok(res)
}

fn del_user(
    db: web::Data<Pool>,
    userid: i32
) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(users.find(userid)).execute(&conn)?;
    Ok(count)
}

fn delete_single_event(
    db: web::Data<Pool>,
    eventid: i32
) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(events.find(eventid)).execute(&conn)?;
    Ok(count)
}


pub async fn desc(db: web::Data<Pool>, eventid: web::Path<i32>) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || get_desc(db, eventid.into_inner()))
        .await
        .map(|ev| HttpResponse::Ok().json(ev))
        .map_err(|_| HttpResponse::InternalServerError())?
    )
}
fn get_desc(db: web::Data<Pool>, eventid: i32) -> Result<String, diesel::result::Error> {
    let conn = db.get().unwrap();
    let res = events.find(eventid).select(description).first::<Option<String>>(&conn)?;
    Ok(res.unwrap())
}

//Question APIs

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionData {
    pub question_description: String,
    pub event_id: i32
}

pub async fn get_questions(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_questions(db))
        .await
        .map(|qu| HttpResponse::Ok().json(qu))
        .map_err(|_| HttpResponse::InternalServerError())?
        )
}

fn get_all_questions(db: web::Data<Pool>) -> Result<Vec<Question>, diesel::result::Error>{
    let conn = db.get().unwrap();
    let res = questions.load::<Question>(&conn)?;
    Ok(res)
}

pub async fn get_question_by_id(db: web::Data<Pool>, questionid: web::Path<i32>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || question_by_id(db, questionid.into_inner()))
        .await
        .map(|qu| HttpResponse::Ok().json(qu))
        .map_err(|_| HttpResponse::InternalServerError())?
        )
}
fn question_by_id(db: web::Data<Pool>, questionid: i32) -> Result<Question, diesel::result::Error> {
    let conn = db.get().unwrap();
    questions.find(questionid).get_result::<Question>(&conn)
}

pub async fn add_question(db: web::Data<Pool>, item: web::Json<QuestionData>) -> Result<HttpResponse, Error>{
    Ok(web::block(move || single_question(db, item))
        .await
        .map(|ques| HttpResponse::Created().json(ques))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn single_question(
    db: web::Data<Pool>,
    questiondata: web::Json<QuestionData> 
    ) -> Result<Question, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_question = NewQuestion {
        question_description: &questiondata.question_description,
        event_id: &questiondata.event_id
    };
    let res = insert_into(questions).values(&new_question).get_result(&conn)?;
    Ok(res)
}

pub async fn delete_question(
    db: web::Data<Pool>,
    eventid: web::Path<i32>
    ) -> Result<HttpResponse, Error> {
    Ok(web::block(move || del_question(db, eventid.into_inner()))
        .await
        .map(|eve| HttpResponse::Ok().json(eve))
        .map_err(|_| HttpResponse::InternalServerError())?
        )
}

fn del_question(
    db: web::Data<Pool>,
    questionid: i32
) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(questions.find(questionid)).execute(&conn)?;
    Ok(count)
}

// Survey Responses

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseData {
    pub user_id: i32,
    pub event_id: i32,
    pub question_id: i32,
    pub response_date: String,
    pub user_response: String,
}

pub async fn get_responses(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_responses(db))
        .await
        .map(|qu| HttpResponse::Ok().json(qu))
        .map_err(|_| HttpResponse::InternalServerError())?
        )
}

fn get_all_responses(db: web::Data<Pool>) -> Result<Vec<Responses>, diesel::result::Error>{
    let conn = db.get().unwrap();
    let res = response.load::<Responses>(&conn)?;
    Ok(res)
}

pub async fn get_response_by_id(db: web::Data<Pool>, response_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || response_by_id(db, response_id.into_inner()))
        .await
        .map(|qu| HttpResponse::Ok().json(qu))
        .map_err(|_| HttpResponse::InternalServerError())?
        )
}
fn response_by_id(db: web::Data<Pool>, response_id: i32) -> Result<Responses, diesel::result::Error> {
    let conn = db.get().unwrap();
    response.find(response_id).get_result::<Responses>(&conn)
}

pub async fn add_response(db: web::Data<Pool>, item: web::Json<ResponseData>) -> Result<HttpResponse, Error>{
    Ok(web::block(move || single_response(db, item))
        .await
        .map(|ques| HttpResponse::Created().json(ques))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn single_response(
    db: web::Data<Pool>,
    responsedata: web::Json<ResponseData> 
    ) -> Result<Responses, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_response = NewResponse {
        user_id: &responsedata.user_id,
        event_id: &responsedata.event_id,
        question_id: &responsedata.question_id,
        response_date: chrono::NaiveDateTime::parse_from_str(&responsedata.response_date, "%Y-%m-%d %H:%M:%S").unwrap(),
        user_response: &responsedata.user_response
    };
    let res = insert_into(response).values(&new_response).get_result(&conn)?;
    Ok(res)
}

pub async fn delete_response(
    db: web::Data<Pool>,
    responseid: web::Path<i32>
    ) -> Result<HttpResponse, Error> {
    Ok(web::block(move || del_response(db, responseid.into_inner()))
        .await
        .map(|eve| HttpResponse::Ok().json(eve))
        .map_err(|_| HttpResponse::InternalServerError())?
        )
}

fn del_response(
    db: web::Data<Pool>,
    responseid: i32
) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(response.find(responseid)).execute(&conn)?;
    Ok(count)
}
