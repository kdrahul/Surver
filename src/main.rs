#[macro_use]
extern crate diesel;

use actix_web::{HttpServer, web, App};
use actix_cors::Cors;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod api;
mod models;
mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;


#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let manager = ConnectionManager::<PgConnection>::new(db_url);   
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .data(pool.clone())
            // All user related api calls goes here
            .service(web::scope("/users")
                .route("/", web::get().to(api::get_users))
                .route("/{id}", web::get().to(api::get_users_by_id))
                .route("/", web::post().to(api::add_users))
                .route("/{id}", web::delete().to(api::delete_users))
                )
            // All event lists
            .service(web::scope("/events")
                .route("/", web::get().to(api::get_events))
                .route("/{id}", web::get().to(api::get_event_by_id))
                .route("/", web::post().to(api::add_event))
                .route("/{id}", web::delete().to(api::delete_event))
            )
            }
        )
        .bind("127.0.0.1:8989")?
        .run()
        .await
}
