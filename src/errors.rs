use ohkami::{Response, IntoResponse};
use worker::send::SendWrapper;
use crate::pages;


pub enum AppError {
    RenderingHTML(yarte::Error),
    Validation(String),
    KV(SendWrapper<worker::kv::KvError>),
    Worker(worker::Error),
}
impl AppError {
    pub fn kv(kv_error: worker::kv::KvError) -> Self {
        Self::KV(SendWrapper(kv_error))
    }
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
