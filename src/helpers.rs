use crate::AppError;
use crate::js;
use std::sync::Arc;
use worker::send::SendWrapper;


#[allow(non_snake_case)]
pub fn AssertSend<T: Send>(f: T) -> T {f}

pub async fn create_key(
    ctx: &worker::Context,
    kv:  worker::kv::KvStore,
    url: &str,
) -> Result<Arc<String>, AppError> {
    worker::console_log!("`created_key` is called");

    let new_key = Arc::new(loop {
        let uuid = js::randomUUID();
        let key  = {
            let mut key = uuid;
            unsafe {key.as_mut_vec().truncate(6)}
            key
        };

        worker::console_log!("generated key: `{key}`");
        
        if kv.get(&key).text().await.map_err(|e| AppError::KV(SendWrapper(e)))?.is_none() {
            break key
        }
    });

    let (key, url) = (new_key.clone(), String::from(url));
    ctx.wait_until(async move {
        match kv.put(&key, url) {
            Err(err) => {
                worker::console_error!("Can't put: {err}");
            },
            Ok(put) => if let Err(err) = put.execute().await {
                worker::console_error!("Failed to put to kv: {err}");
            }
        };
    });

    Ok(new_key)
}
