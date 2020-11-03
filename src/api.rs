use actix_web::{web, HttpResponse, Responder};

pub async fn index() -> impl Responder {
    format!("Hello")
//    Ok(web::block(move || Responder))
 //       .await
  //      .map_err(|_| HttpResponse::InternalServerError())
}
