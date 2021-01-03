use super::models::{Users, NewUser, Events, NewEvent};
use diesel::dsl::count_star;
use super::schema::users::dsl::*;
use super::schema::events::dsl::*;
use actix_web::{web, HttpResponse, Error };
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
    pub organizers: String,
    pub max_limit: i32,
    pub fee: i32,
    pub prize_money: i32,
    pub venue: String,
    pub starts_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionData {
    pub question: String,
    pub option1: String,
    pub option2: String,
    pub option3: String,
    pub option4: String,
    pub answer: i32,
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
    user_id: web::Path<i32>
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || user_by_id(db, user_id.into_inner()))
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
    user_id: web::Path<i32>
    ) -> Result<HttpResponse, Error> {
    Ok(web::block(move || del_user(db, user_id.into_inner()))
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

pub async fn get_event_by_id(db: web::Data<Pool>, event_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || get_single_event(db, event_id.into_inner()))
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
    event_id: web::Path<i32>
    ) -> Result<HttpResponse, Error> {
    Ok(web::block(move || delete_single_event(db, event_id.into_inner()))
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

fn get_single_event(db: web::Data<Pool>, event_id: i32) -> Result<Events, diesel::result::Error> {
    let conn = db.get().unwrap();
    let res = events.find(event_id).get_result::<Events>(&conn)?;
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

fn user_by_id(pool: web::Data<Pool>, user_id: i32) -> Result<Users, diesel::result::Error> {
    let conn = pool.get().unwrap();
    users.find(user_id).get_result::<Users>(&conn)
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
        starts_at: chrono::Local::now().naive_local(),
        max_limit: &eventdata.max_limit,
        fee: &eventdata.fee,
        prize_money: &eventdata.prize_money,
        venue: &eventdata.venue,
    };
    let res = insert_into(events).values(&new_event).get_result(&conn)?;
    Ok(res)
}

fn del_user(
    db: web::Data<Pool>,
    user_id: i32
) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(users.find(user_id)).execute(&conn)?;
    Ok(count)
}

fn delete_single_event(
    db: web::Data<Pool>,
    event_id: i32
) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(events.find(event_id)).execute(&conn)?;
    Ok(count)
}


pub async fn desc(db: web::Data<Pool>, event_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || get_desc(db, event_id.into_inner()))
        .await
        .map(|ev| HttpResponse::Ok().json(ev))
        .map_err(|_| HttpResponse::InternalServerError())?
    )
}
fn get_desc(db: web::Data<Pool>, event_id: i32) -> Result<String, diesel::result::Error> {
    let conn = db.get().unwrap();
    let res = events.find(event_id).select(description).first::<Option<String>>(&conn)?;
    Ok(res.unwrap())
}
// API for /events
//pub async fn get_surveys() -> Result<HttpResponse, Error>{} 
//pub async fn add_surveys() -> Result<HttpResponse, Error>{} 
//pub async fn delete_surveys() -> Result<HttpResponse, Error>{} 
//pub async fn user_to_event() -> Result<HttpResponse, Error>{} 
