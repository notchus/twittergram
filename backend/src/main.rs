use actix_web::{get, App, HttpServer, Responder, HttpResponse};
use std::env;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Superfast App backend is running 🚀")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Read DATABASE_URL to ensure it's available
    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    println!("📦 DATABASE_URL loaded: {}", db_url);

    HttpServer::new(|| App::new().service(index))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}