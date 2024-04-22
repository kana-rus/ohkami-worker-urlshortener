use ohkami::prelude::*;
use yarte::Template;
use crate::AppError;


#[derive(Clone)]
pub struct LayoutFang;
impl FangAction for LayoutFang {
    async fn back<'a>(&'a self, res: &'a mut Response) {
        #[derive(Template)]
        #[template(src = r#"<!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <link rel="stylesheet" href="https://fonts.xz.style/serve/inter.css" />
            <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@exampledev/new.css@1.1.2/new.min.css" />
            <title>URL Shortener</title>
        </head>
        <body>
            <header>
                <h1>
                    <a href="/">URL Shortener</a>
                </h1>
            </header>
            <div>{{{ inner }}}</div>
        </body>
        </html>"#)]
        struct Layout { inner: String }

        if res.headers.ContentType().is_some_and(|ct| ct.starts_with("text/html")) {
            let inner = res.drop_content().map(|bytes|
                String::from_utf8(bytes.into_owned()).unwrap()
            ).unwrap_or_else(String::new);

            match (Layout { inner }.call()) {
                Ok(html) => res.set_html(html),
                Err(err) => *res = AppError::RenderingHTML(err).into_response(),
            }
        }
    }
}
