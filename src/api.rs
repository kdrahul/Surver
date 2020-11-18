use super::models::{ERSUsers, NewUser};
use super::schema::ersusers::dsl::*;
use actix_web::{web, HttpResponse, Error };
use super::Pool;
use serde::{Serialize, Deserialize};
use diesel::dsl::{insert_into, delete};
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use std::vec::Vec;


#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub user_group: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub branch: String,
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

fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<ERSUsers>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = ersusers.load::<ERSUsers>(&conn)?;
    Ok(items)
}

fn user_by_id(pool: web::Data<Pool>, user_id: i32) -> Result<ERSUsers, diesel::result::Error> {
    let conn = pool.get().unwrap();
    ersusers.find(user_id).get_result::<ERSUsers>(&conn)
}

fn single_user(
    db: web::Data<Pool>,
    userdata: web::Json<UserData> 
    ) -> Result<ERSUsers, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_user = NewUser {
        name: &userdata.name,
        user_group: &userdata.user_group,
        email: &userdata.email,
        phone:  &userdata.phone,
        branch: &userdata.branch,
        joined_on: chrono::Local::now().naive_local()
    };
    let res = insert_into(ersusers).values(&new_user).get_result(&conn)?;
    Ok(res)
}

fn del_user(
    db: web::Data<Pool>,
    user_id: i32
) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(ersusers.find(user_id)).execute(&conn)?;
    Ok(count)
}



// API for /events
//pub async fn get_surveys() -> Result<HttpResponse, Error>{} 
//pub async fn add_surveys() -> Result<HttpResponse, Error>{} 
//pub async fn delete_surveys() -> Result<HttpResponse, Error>{} 
//pub async fn user_to_event() -> Result<HttpResponse, Error>{} 
