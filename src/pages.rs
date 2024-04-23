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
