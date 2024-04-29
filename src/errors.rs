use ohkami::{Response, IntoResponse};
use crate::pages;


pub enum AppError {
    RenderingHTML(yarte::Error),
    Validation(String),
    KV(worker::kv::KvError),
    Worker(worker::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            Self::RenderingHTML(err) => {
                worker::console_error!("Failed to render HTML: {err}");
                Response::InternalServerError()
            }
            Self::Validation(msg) => {
                worker::console_error!("Validation failed: {msg}");
                pages::ErrorPage.into_response()
            }
            Self::KV(err) => {
                worker::console_error!("Error from KV: {err}");
                Response::InternalServerError()
            }
            Self::Worker(err) => {
                worker::console_error!("Error in Worker: {err}");
                Response::InternalServerError()
            }
        }
    }
}

impl From<worker::Error> for AppError {
    fn from(e: worker::Error) -> Self {
        Self::Worker(e)
    }
}
impl From<worker::kv::KvError> for AppError {
    fn from(e: worker::kv::KvError) -> Self {
        Self::KV(e)
    }
}
