use std::{env, net::SocketAddr};

use axum::{extract::{Path, Query, State}, http::StatusCode, response::{Html, IntoResponse, Redirect, Response}, routing::{get, post}, serve, Json, Router};
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tracing::{ error, info};
mod error;
use crate::model::{Formurl, ModelControllerDB, ModelControllerRAM};
mod model;
#[cfg(test)]
mod test;
#[derive(Clone)]
pub struct ModelController{
    pub db: ModelControllerDB,
    pub ram: ModelControllerRAM,
}
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let url=env::var("DATABASE_url").expect("DB url not found");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("");
    let db= ModelControllerDB::new(pool).await;
    let ram = ModelControllerRAM::new().await;
    let mc= ModelController{db, ram};
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(hello))
        .route("/:url", get(fetchurl))
        .route("/api/shortner", post(shortenurl))
        .with_state(mc);
    let addr = SocketAddr::from(([127,0,0,1], 8080));
    info!("lisening on port http://{addr}");
    let connect = TcpListener::bind(&addr).await.unwrap();
    serve(connect, app).await.unwrap();
}
#[derive(Deserialize)]
struct Qdata{
    name: Option<String>,
}

async fn hello(Query(data): Query<Qdata>) -> impl IntoResponse{
    let name = data.name.unwrap_or("world!!".to_string());
    Html(format!("hello, {name}"))
}
#[axum::debug_handler]
async fn shortenurl(State(mc): State<ModelController>, Json(data): Json<Formurl>) -> Result<impl IntoResponse, Response>{
    info!("Received request to shorten URL");

    match mc.db.shortenurl(data, &mc).await{
        Ok(url) => {
            info!("suceess operation {url:?}");
            Ok(Json(url))

        }
        Err(err) => {
            error!("Database error: {err:?}");
            Err(err.to_string().into_response())
        }
    }
    
}
async fn fetchurl(Path(url): Path<String>, State(mc): State<ModelController>) -> impl IntoResponse {
    match mc.db.fetchurl(url).await {
        Ok(long_url) => Redirect::temporary(&long_url).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Short URL not found").into_response(),
    }
}