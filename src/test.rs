#![allow(unused_imports, dead_code)]
use std::env;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::{get, post},
    Router,

};
use axum::body::to_bytes;
use tower::util::ServiceExt; 
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;

use crate::{hello, model::{ModelControllerDB, ModelControllerRAM}, shortenurl, ModelController};

#[tokio::test]
async fn test_shorten_url_handler() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to DB");
    let db= ModelControllerDB::new(pool).await;
    let ram = ModelControllerRAM::new().await;
    let mc= ModelController{db, ram};
    let app = Router::new()
        .route("/", get(hello))
        .route("/api/shortner", post(shortenurl))
        .with_state(mc);

    let payload = r#"{"long_url": "https://example.com"}"#;
    let request = Request::builder()
        .method("POST")
        .uri("/api/shortner")
        .header("content-type", "application/json")
        .body(Body::from(payload))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = to_bytes(response.into_body(), 64 * 1024)
        .await
        .unwrap();
    println!("Response: {}", String::from_utf8_lossy(&body_bytes));
}
