#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::arbitrary_source_item_ordering)]
#![allow(clippy::question_mark_used)]
#![allow(clippy::str_to_string)]
#![allow(clippy::absolute_paths)]
#![allow(clippy::module_name_repetitions)]

use std::{env, net::SocketAddr};

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
    serve,
};
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tracing::{debug, error, info};
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
async fn main() {
    dotenvy::dotenv().ok();
    let url = env::var("DATABASE_url").unwrap_or_else(|err| {
        error!(" Environment variable `DATABASE_url` not found.{err}");
        std::process::exit(1);
    });
    let pool = match PgPoolOptions::new().max_connections(5).connect(&url).await {
        Ok(pg) => pg,
        Err(err) => {
            error!("internel server error {err}");
            std::process::exit(1)
        }
    };
    let db = ModelControllerDB::new(pool);
    let ram = ModelControllerRAM::default();
    let mc = ModelController { db, ram };
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(hello))
        .route("/:url", get(fetchurl))
        .route("/api/shortner", post(shorten_url))
        .with_state(mc);
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("lisening on port http://{addr}");
    let connect = match TcpListener::bind(&addr).await {
        Ok(lis) => lis,
        Err(err) => {
            error!("internel server error {err}");
            std::process::exit(1);
        }
    };
    serve(connect, app)
        .await
        .unwrap_or_else(|err| debug!("internel server error {err}"));
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
) -> Result<impl IntoResponse, Response> {
    info!("Received request to shorten URL");

    match mc.db.shorten_url(data, &mc).await {
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
    (mc.db.fetchurl(url).await).map_or_else(
        |_| (StatusCode::NOT_FOUND, "Short URL not found").into_response(),
        |long_url| Redirect::temporary(&long_url).into_response(),
    )
}
