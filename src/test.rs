#![allow(unused_imports, dead_code)]
use axum::body::to_bytes;
use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::{get, post},
};
use bytes::Bytes;
use dotenvy::dotenv;
use serde::de::Error;
use sqlx::postgres::PgPoolOptions;
use std::{env, process::exit};
use tower::util::ServiceExt;
use tracing::{debug, error, info};

use crate::{
    ModelController, hello,
    model::{ModelControllerDB, ModelControllerRAM},
    shorten_url,
};

#[tokio::test]
async fn test_shorten_url_handler() {
    dotenv().ok();
    let url = env::var("DATABASE_url").unwrap();
    let pool = PgPoolOptions::new().max_connections(5).connect(&url).await.unwrap();
    let db = ModelControllerDB::new(pool);
    let ram = ModelControllerRAM::default();
    let mc = ModelController { db, ram };
    let app = Router::new()
        .route("/", get(hello))
        .route("/api/shortner", post(shorten_url))
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
    info!("Response: {}", String::from_utf8_lossy(&body_bytes));
}
