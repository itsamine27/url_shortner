#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::arbitrary_source_item_ordering)]
#![allow(clippy::question_mark_used)]
#![allow(clippy::str_to_string)]
#![allow(clippy::absolute_paths)]
#![allow(clippy::module_name_repetitions)]

use std::net::SocketAddr;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
};
use dotenvy::dotenv;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use crate::error::Result;
mod error;
use crate::model::{Formurl, ModelControllerDB, ModelControllerRAM};
mod model;
#[cfg(test)]
mod test;
#[derive(Clone)]
pub struct ModelController {
    pub db: ModelControllerDB,
    pub ram: ModelControllerRAM,
}
#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;
    let db  = ModelControllerDB::new(pool);
    let ram = ModelControllerRAM::default();
    let mc  = ModelController { db, ram };

    // 3. init tracing AFTER building state
    tracing_subscriber::fmt::init();

    // 4. build app
    let app = Router::new()
        .route("/", get(hello))
        .route("/:url", get(fetchurl))
        .route("/api/shortner", post(shorten_url))
        .with_state(mc);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
#[derive(Deserialize)]
struct Qdata {
    name: Option<String>,
}

async fn hello(Query(data): Query<Qdata>) -> impl IntoResponse {
    let name = data.name.unwrap_or_else(|| "world!!".to_string());
    Html(format!("hello, {name}"))
}
async fn shorten_url(
    State(mc): State<ModelController>,
    Json(data): Json<Formurl>,
) -> Result<impl IntoResponse> {
    info!("Received request to shorten URL");

    Ok(Json(mc.db.shorten_url(data, &mc).await?))
}
async fn fetchurl(Path(url): Path<String>, State(mc): State<ModelController>) -> Result<impl IntoResponse> {
    let url = mc.db.fetchurl(url).await?;
    Ok(Redirect::temporary(&url))
}