use crate::AppError;
use yarte::Template;
use ohkami::{IntoResponse, Response};


macro_rules! page {
    ($name:ident = ($({$( $field:ident: $t:ty ),*})? $(;$semi:tt)?) => $template:literal) => {
        #[derive(Template)]
        #[template(src = $template)]
        pub struct $name $({
            $( pub $field: $t ),*
        })? $($semi)?

        impl IntoResponse for $name {
            fn into_response(self) -> Response {
                match self.call() {
                    Ok(html) => Response::OK().with_html(html),
                    Err(err) => AppError::RenderingHTML(err).into_response(),
                }
            }
        }
    };
}

page!(Layout = ({ content: String }) => r#"<!DOCTYPE html>
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
        <div>{{{ content }}}</div>
    </body>
    </html>"#
);

page!(IndexPage = (;;) => r#"
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
"#);

page!(CreatedPage = ({ shorten_url: String }) => r#"
    <div>
        <h2>Created!</h2>
        <a href="{{ shorten_url }}">
            {{ shorten_url }}
        </a>
    </div>
"#);

page!(ErrorPage = (;;) => r#"
    <div>
        <h2>Error!</h2>
        <a href="/">Back to top</a>
    </div>
"#);
