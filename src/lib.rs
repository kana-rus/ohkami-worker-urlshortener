mod errors;
mod fangs;
mod js;
mod models;
mod pages;

use errors::AppError;
use fangs::{LayoutFang, CSRFang};
use models::{IndexPage, CreatedPage, CreateShortenURLForm, KV};

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
        "/:shorten_url"
            .GET(redirect_from_shorten_url),
        "/create".By(Ohkami::with(CSRFang,
            "/".POST(create),
        ))
    ))
}

async fn index() -> IndexPage {
    IndexPage
}

async fn redirect_from_shorten_url(shorten_url: &str,
    kv: KV,
) -> Result<status::Found, AppError> {
    match kv.get(shorten_url).await? {
        Some(url) => Ok(status::Found::at(url)),
        None      => Ok(status::Found::at("/")),
    }
}

async fn create(
    kv:   KV,
    form: CreateShortenURLForm<'_>,
) -> Result<CreatedPage, AppError> {
    if let Err(_) = worker::Url::parse(&form.url) {
        return Err(AppError::Validation(format!("Invalid URL: {}", form.url)))
    }

    worker::console_log!("Got form: {form:?}");

    let key = loop {
        let key = std::sync::Arc::new({
            let mut uuid = js::randomUUID();
            unsafe { uuid.as_mut_vec().truncate(6) }
            uuid
        });
        if kv.get(&*key).await?.is_none() {
            break key
        }
    };
    kv.put(&key.clone(), form.url).await?;
    
    Ok(CreatedPage {
        shorten_url: format!("{ORIGIN}/{key}"),
    })
}
