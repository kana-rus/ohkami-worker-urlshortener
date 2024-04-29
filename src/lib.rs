mod errors;
mod fangs;
mod js;
mod models;
mod pages;

use errors::AppError;
use fangs::{LayoutFang, CSRFang};
use models::{IndexPage, CreatedPage, CreateShortenURLForm, Worker};

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
    w: Worker<'_>,
) -> Result<status::Found, AppError> {
    let kv = w.KV("KV")?;
    match kv.get(key).await? {
        Some(url) => Ok(status::Found::at(url)),
        None      => Ok(status::Found::at("/")),
    }
}

#[worker::send]
async fn create(
    w:    Worker<'_>,
    form: CreateShortenURLForm<'_>,
) -> Result<CreatedPage, AppError> {
    if let Err(_) = worker::Url::parse(&form.url) {
        return Err(AppError::Validation(format!("Invalid URL: {}", form.url)))
    }

    worker::console_log!("Got form: {form:?}");

    let kv = w.KV("KV")?;
    let key = loop {
        let key = std::sync::Arc::new(
            js::randomUUID().split_off(6)
        );
        if kv.get(&*key).await?.is_none() {
            break key
        }
    };
    kv.put(&key.clone(), form.url).await?;
    
    Ok(CreatedPage {
        shorten_url: format!("{ORIGIN}/{key}"),
    })
}
