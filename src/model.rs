use crate::ModelController;
use crate::error::{Error, Result as myRes};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::Row;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering::Acquire};
#[derive(Serialize, Debug)]
pub struct UrlManager {
    id: i32,
    long_url: String,
    short_url: String,
}
#[derive(Deserialize, Serialize)]
pub struct Formurl {
    long_url: String,
}
#[derive(Clone)]
pub struct ModelControllerDB {
    pub pool: PgPool,
}

impl ModelControllerDB {
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn shorten_url(&self, data: Formurl, mc: &ModelController) -> myRes<UrlManager> {
        let short_url: String = mc.ram.generate_short_url()?;
        println!("{short_url:?}sfds");
        let store = sqlx::query_as!(
            UrlManager,
            "
            INSERT INTO url (long_url, short_url)
            VALUES ($1, $2)
            RETURNING id, long_url, short_url
            ",
            data.long_url,
            short_url,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(store)
    }
    pub async fn fetchurl(&self, url: String) -> myRes<String> {
        let store = sqlx::query(
            "
            SELECT long_url
            FROM url
            WHERE short_url = $1
            ",
        )
        .bind(url)
        .fetch_one(&self.pool)
        .await?;
        Ok(store.try_get(0)?)
    }
}

#[derive(Clone)]
pub struct ModelControllerRAM {
    pub inner: Arc<AtomicU64>,
}
impl Default for ModelControllerRAM {
    fn default() -> Self {
        Self::new()
    }
}
const TABLE: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
impl ModelControllerRAM {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn generate_short_url(&self) -> myRes<String> {
        let data = self.inner.fetch_add(1, Acquire);
        let url: Vec<u8> = (0..=5)
            .rev()
            .map(|n| TABLE[((data / 62u64.pow(n)) % 62) as usize])
            .collect();
        match String::from_utf8(url) {
            Ok(sto) => Ok(sto),
            Err(_) => Err(Error::Urlinvalid),
        }
    }
}
