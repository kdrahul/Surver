use actix_web::{HttpServer, web, App};

mod api;

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    dotenv::dotenv().ok();
    //let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(api::index))
    }
        )
        .bind("127.0.0.1:8989")?
        .run()
        .await
}
