#[macro_use]
extern crate diesel;

use actix_web::{HttpServer, web, App, dev::ServiceRequest, Error};
use actix_cors::Cors;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use actix_web_httpauth::extractors::basic::{BasicAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;

mod api;
mod models;
mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn basic_auth_validator(req: ServiceRequest, credentials: BasicAuth) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);

    match validate_credentials(credentials.user_id(), credentials.password().unwrap().trim()) {
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

fn validate_credentials(user_id: &str, user_password: &str) -> Result<bool, std::io::Error> {
    if (user_id.eq_ignore_ascii_case("rahul")) && (user_password.eq_ignore_ascii_case("password")) {
        return Ok(true);
    }
    Err(std::io::Error::new(std::io::ErrorKind::Other, "Auth Failed"))
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
        let auth = HttpAuthentication::basic(basic_auth_validator);
        App::new()
//            .wrap(auth)
            .wrap(Cors::permissive())
            .data(pool.clone())
            .route("/event_count", web::get().to(api::get_event_count))
            .route("/events", web::get().to(api::get_events))
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
                .route("/{id}/description", web::get().to(api::desc))
                .route("/", web::post().to(api::add_event))
                .route("/{id}", web::delete().to(api::delete_event))
            )
            }
        )
        .bind("127.0.0.1:8989")?
        .run()
        .await
}
