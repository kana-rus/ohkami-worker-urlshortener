mod errors;
use std::borrow::Cow;

use errors::AppError;

mod fangs;
use fangs::LayoutFang;
#[cfg(feature="DEBUG")]
use fangs::LoggerFang;

mod helpers;
use helpers::{create_key, AssertSend};

use ohkami::prelude::*;
use ohkami::typed::Payload;
use ohkami::builtin::payload::URLEncoded;
use yarte::Template;
use worker::Url;


#[ohkami::worker]
async fn my_worker() -> Ohkami {
    #[cfg(feature = "DEBUG")]
    console_error_panic_hook::set_once();

    #[cfg(not(feature="DEBUG"))]
    let fangs = LayoutFang;
    #[cfg(feature="DEBUG")]
    let fangs = (LoggerFang, LayoutFang);

    Ohkami::with(fangs, (
        "/".GET(index),
        "/create".POST(create),
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

    worker::console_log!("Got URL: {}", &form.url);

    let key = AssertSend(worker::send::SendFuture::new(create_key(
        ctx,
        env.kv("KV").unwrap(),
        &form.url
    ))).await?;
    
    Ok(CreatedPage {
        shorten_url: format!("/{key}"),
    })
}
