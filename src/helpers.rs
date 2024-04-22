use crate::AppError;
use uuid::Uuid;
use worker::send::{SendFuture, SendWrapper};


pub async fn create_key(
    kv:  worker::kv::KvStore,
    url: &str,
) -> Result<String, AppError> {
    loop {
        let uuid = Uuid::new_v4();
        let key  = String::from_utf8(uuid.as_bytes()[0..6].into()).unwrap();
        
        if kv.get(&key).text().await.map_err(|e| AppError::KV(SendWrapper(e)))?.is_none() {
            SendFuture::new(
                kv.put(&key, url).map_err(|e| AppError::KV(SendWrapper(e)))?
                .execute()).await.map_err(|e| AppError::KV(SendWrapper(e)))?;
            break Ok(key)
        }
    }
}

#[allow(non_snake_case)]
pub fn AssertSend<T: Send>(f: T) -> T {f}
