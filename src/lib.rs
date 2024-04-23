mod errors;
mod fangs;
mod helpers;
mod js;

use errors::AppError;
use fangs::LayoutFang;
use helpers::{create_key, get_original_url};

use ohkami::prelude::*;
use ohkami::typed::{Payload, status};
use ohkami::builtin::payload::URLEncoded;
use yarte::Template;
use worker::Url;
use std::borrow::Cow;


#[ohkami::worker]
async fn my_worker() -> Ohkami {
    #[cfg(feature = "DEBUG")]
    console_error_panic_hook::set_once();

    Ohkami::with(LayoutFang, (
        "/"
            .GET(index),
        "/create"
            .POST(create),
        "/:shorten_url"
            .GET(redirect_from_shorten_url),
    ))
}


#[derive(Template)]
#[template(src=r#"
<div>
    <h2>Create shorten URL!</h2>
    <form action="/create" method="post">
        <input
            type="text"
            name="url"
            autocomplete="off"
            style="width: 80%;"
        />
        &nbsp;
        <button type="submit">Create</button>
    </form>
</div>
"#)]
struct IndexPage;
impl IntoResponse for IndexPage {
    fn into_response(self) -> Response {
        match self.call() {
            Ok(html) => Response::OK().with_html(html),
            Err(err) => AppError::RenderingHTML(err).into_response(),
        }
    }
}

async fn index() -> IndexPage {
    IndexPage
}


#[Payload(URLEncoded/D)]
#[derive(Debug)]
struct CreateShortenURLForm<'req> {
    url: Cow<'req, str>,
}

#[derive(Template)]
#[template(src=r#"
<div>
    <h2>Created!</h2>
    <input
        autofocus
        type="text"
        value="{{ shorten_url }}"
        style="width: 80%;"
    />
</div>
"#)]
struct CreatedPage {
    shorten_url: String,
}
impl IntoResponse for CreatedPage {
    fn into_response(self) -> Response {
        match self.call() {
            Ok(html) => Response::OK().with_html(html),
            Err(err) => AppError::RenderingHTML(err).into_response(),
        }
    }
}

async fn create(
    ctx:  &worker::Context,
    env:  &worker::Env,
    form: CreateShortenURLForm<'_>,
) -> Result<CreatedPage, AppError> {
    Url::parse(&form.url).map_err(|e| AppError::Validation(format!(
        "Invalid URL: {e}"
    )))?;

    worker::console_log!("Got form: {form:?}");

    let key = worker::send::SendFuture::new(
        create_key(
            ctx,
            env.kv("KV").unwrap(),
            &form.url
        )
    ).await?;
    
    Ok(CreatedPage {
        shorten_url: format!("/{key}"),
    })
}


async fn redirect_from_shorten_url(
    shorten_url: &str,
    env:         &worker::Env,
) -> Result<status::Found, AppError> {
    match worker::send::SendFuture::new(
        get_original_url(
            env.kv("KV").unwrap(),
            shorten_url
        )
    ).await? {
        Some(url) => Ok(status::Found::at(url)),
        None      => Ok(status::Found::at("/")),
    }
}
