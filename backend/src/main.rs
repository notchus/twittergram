mod db;

use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use std::env;
use deadpool_postgres::Pool;

#[get("/")]
async fn index(db_pool: web::Data<Pool>) -> impl Responder {
    // Example: ensure the connection works
    let client = db_pool.get().await.expect("DB connection failed");
    let row = client.query_one("SELECT 1 + 1", &[]).await.unwrap();
    let val: i32 = row.get(0);

    HttpResponse::Ok().body(format!("Superfast App backend is running 🚀 DB test: {}", val))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    let db_pool = db::create_pool(&db_url);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .service(index)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
