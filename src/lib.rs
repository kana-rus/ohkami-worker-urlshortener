mod errors;
mod fangs;
mod js;
mod models;
mod pages;

use errors::AppError;
use fangs::{LayoutFang, CSRFang};
use models::{IndexPage, CreateShortenURLForm, CreatedOrErrorPage, KV};

use ohkami::prelude::*;
use ohkami::typed::status;
use worker::Url;
use std::sync::Arc;


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
    // ctx:  &worker::Context,
    form: CreateShortenURLForm<'_>,
) -> Result<CreatedOrErrorPage, AppError> {
    if let Err(_) = Url::parse(&form.url) {
        return Ok(CreatedOrErrorPage::Error)
    }

    worker::console_log!("Got form: {form:?}");

    let key = loop {
        let mut uuid = js::randomUUID();
        unsafe { uuid.as_mut_vec().truncate(6) }
        let key = Arc::new(uuid);

        if kv.get(&*key).await?.is_none() {
            break key
        }
    };

    // // ctx.wait_until({
    // {
    //     let key = Arc::clone(&key);
    //     let url = String::from(form.url);
    //     // async move {
    //         // if let Err(err) = kv.put(&*key.clone(), url).await {
    //         //     worker::console_error!("...");
    //         // }
    //         kv.put(&*key, value)
    //     // }
    // }
    // });
    kv.put(&*Arc::clone(&key), form.url).await?;
    
    Ok(CreatedOrErrorPage::Created {
        shorten_url: format!("https://ohkami-urlshortener.kanarus.workers.dev/{key}"),
    })
}
