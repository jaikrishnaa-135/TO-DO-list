use actix_files::Files;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_cors::Cors;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};
use std::env;

#[derive(Serialize, Deserialize)]
struct Test {
    id: i32,
    title: String,
    completed: Option<i8>,
}

#[derive(Deserialize)]
struct NewTest {
    title: String,
}


#[derive(Debug)]
#[derive(Deserialize)]
struct UpdateTestt {
    completed: Option<i8>,
}

async fn update_test(
    pool: web::Data<MySqlPool>,
    test_id: web::Path<i32>,
    update: web::Json<UpdateTestt>,
) -> impl Responder {
    // let id = test_id.into_inner();
    let _completed = update.completed.unwrap_or(0);
    println!("{} {:?}",test_id,update);
    let result = sqlx::query!(
        "UPDATE tests SET completed = ? WHERE id = ?",
        update.completed,
        *test_id)
    .execute(pool.get_ref())    
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Test updated"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update test"),
    }
}



async fn get_tests(pool: web::Data<MySqlPool>) -> impl Responder {
    let tests = sqlx::query_as!(
        Test,
        "SELECT id, title, completed FROM tests"
    )
    .fetch_all(pool.get_ref())

    .await
    .unwrap();

    HttpResponse::Ok().json(tests)
}

async fn add_test(
    pool: web::Data<MySqlPool>,
    new_test: web::Json<NewTest>,
) -> impl Responder {
    sqlx::query!(
        "INSERT INTO tests (title) VALUES (?)",
        new_test.title
    )
    .execute(pool.get_ref())
    .await
    .unwrap();

    HttpResponse::Ok().body("Test added")
}

async fn delete_test(
    pool: web::Data<MySqlPool>,
    test_id: web::Path<i32>,
) -> impl Responder {
    println!("{}",test_id);
    let result = sqlx::query!(
        "DELETE FROM tests WHERE id = ?",
        *test_id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Test deleted"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete test"),
    }
}

async fn test_from_rust() -> impl Responder {
    HttpResponse::Ok().body("Hello from Rust API")
}

#[actix_web::main]


async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = MySqlPoolOptions::new()
        .connect(&database_url)         
        .await
        .expect("Could not connect to MySQL");

    // Get PORT from environment, default to 8080 if not set (useful for local dev)
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    println!("Server running on http://{}", addr);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(pool.clone()))
            .route("/api/testing", web::get().to(test_from_rust))
            .route("/api/tests", web::get().to(get_tests))
            .route("/api/tests", web::post().to(add_test))
            .route("/api/tests/{id}", web::delete().to(delete_test))
            .route("/api/tests/complete/{id}", web::put().to(update_test))
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind(addr)?  // Now binding to 0.0.0.0:<PORT>
    .run()
    .await
}
