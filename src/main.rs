#[macro_use]
extern crate diesel;

use actix_web::{HttpServer,Error, web, App,dev::ServiceRequest };
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod api;
mod auth;
mod models;
mod schema;
mod errors;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.get_ref().clone())
        .unwrap_or_else(Default::default);
    match auth::validate_token(credentials.token()) {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}

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
        let auth = HttpAuthentication::bearer(validator);
        App::new()
            .wrap(auth)
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
                //.route("/", web::get().to(api::get_surveys))
                //.route("/", web::post().to(api::add_surveys))
                //.route("/{id}", web::delete().to(api::delete_surveys))
            )
            .service(web::scope("/registration")
                //.route("/{event_id}", web::get().to(api::user_to_event))
                )
            }
        )
        .bind("127.0.0.1:8989")?
        .run()
        .await
}
