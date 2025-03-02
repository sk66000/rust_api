use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use std::env;
use actix_web::{web, App, HttpServer, HttpResponse, Error};

use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;

mod routes;
mod models;
mod auth;

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result <ServiceRequest, Error> {
    let token = credentials.token();
    match auth::validate_jwt(token) {
        Ok(_) => Ok(req),
        Err(_) => Err(ErrorUnauthorized("Invalid token")),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result <()> {

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(auth)
            .service(web::scope("/api")
                .service(routes::create_item)
                .service(routes::get_item)
                .service(routes::update_item)
                .service(routes::delete_item)
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await

}


