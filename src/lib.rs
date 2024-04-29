mod errors;
mod fangs;
mod js;
mod models;
mod pages;

use errors::AppError;
use fangs::{LayoutFang, CSRFang};
use models::{IndexPage, CreatedPage, CreateShortenURLForm};

use ohkami::prelude::*;
use ohkami::typed::status;

const ORIGIN: &str = if cfg!(feature = "DEBUG") {
    "http://localhost:8787"
} else {
    "https://ohkami-urlshortener.kanarus.workers.dev"
};

#[ohkami::worker]
async fn my_worker() -> Ohkami {
    #[cfg(feature = "DEBUG")]
    console_error_panic_hook::set_once();

    Ohkami::with(LayoutFang, (
        "/"
            .GET(index),
        "/:key"
            .GET(redirect),
        "/create".By(Ohkami::with(CSRFang,
            "/".POST(create),
        ))
    ))
}

async fn index() -> IndexPage {
    IndexPage
}

#[worker::send]
async fn redirect(key: &str,
    env: &worker::Env,
) -> Result<status::Found, AppError> {
    let kv = env.kv("KV").map_err(AppError::Worker)?;
    match kv.get(key).text().await.map_err(AppError::kv)? {
        Some(url) => Ok(status::Found::at(url)),
        None      => Ok(status::Found::at("/")),
    }
}

#[worker::send]
async fn create(
    env: &worker::Env,
    form: CreateShortenURLForm<'_>,
) -> Result<CreatedPage, AppError> {
    if let Err(_) = worker::Url::parse(&form.url) {
        return Err(AppError::Validation(format!("Invalid URL: {}", form.url)))
    }

    worker::console_log!("Got form: {form:?}");

    let kv = env.kv("KV").map_err(AppError::Worker)?;

    let key = loop {
        let key = std::sync::Arc::new({
            let mut uuid = js::randomUUID();
            unsafe { uuid.as_mut_vec().truncate(6) }
            uuid
        });
        if kv.get(&*key).text().await.map_err(AppError::kv)?.is_none() {
            break key
        }
    };
    kv.put(&key.clone(), form.url).map_err(AppError::kv)?
        .execute().await.map_err(AppError::kv)?;
    
    Ok(CreatedPage {
        shorten_url: format!("{ORIGIN}/{key}"),
    })
}
