
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio::sync::Mutex;
use tracing::error;
use crate::error::{Result as myRes, Error};
use crate::ModelController;
use sqlx::Row;
#[derive(Serialize, Debug)]
pub struct UrlManager{
    id: i32,
    long_url: String,
    short_url: String,
}
#[derive(Deserialize, Serialize)]
pub struct Formurl{
    long_url: String,
}
#[derive(Clone)]
pub struct ModelControllerDB{
    pub pool: PgPool,
}

impl ModelControllerDB{
    pub const fn new(pool: PgPool) -> Self{
        Self{pool}
    }
    pub async fn shorten_url(&self, data: Formurl, mc:&ModelController) -> myRes<UrlManager>{

        let short_url:String = mc.ram.generate_short_url().await?;
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
    pub async fn fetchurl (&self, url:String)->myRes<String>{
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
pub struct ModelControllerRAM{
    pub inner: Arc<Mutex<String>>,
}
impl Default for ModelControllerRAM{
    fn default() -> Self {
        Self::new()
    }
}
impl ModelControllerRAM{
    pub fn new()->Self{
        Self{
            inner: Arc::new(Mutex::new("000000".to_string()))
        }
    }
    #[allow(unused_assignments)]
    pub async fn generate_short_url(&self)->myRes<String>{
        let mut storeg = self.inner.lock().await;
        let mut data = storeg.clone().into_bytes();
        let mut next= 1;
        let len = data.len();
        loop  {
            let last = data.get_mut(len-next);
            if let Some(last) = last{
                match last{
                    b'9' => {
                        *last = b'a';
                    }
                    b'z' => {
                        *last= b'A';
                    }
                    b'Z' => {
                        *last= b'0';
                        next+=1;
                        continue;
                    }
                    _ =>{
                        *last +=1;
                    }
                    
                }
            }  
            break;
        }
        let res=String::from_utf8(data).unwrap_or_else(|err|{
            error!("an error has occured {err}");
            String::new()
        });
        if res == *storeg{
           return Err(Error::Urlinvalid);
        }
        (*storeg).clone_from(&res);
        drop(storeg);
        Ok(res)
    }
}
