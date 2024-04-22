use ohkami::{Response, IntoResponse};
use worker::send::SendWrapper;
use crate::helpers::*;


pub enum AppError {
    RenderingHTML(yarte::Error),
    Validation(String),
    KV(SendWrapper<worker::kv::KvError>),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match AssertSend(self) {
            Self::RenderingHTML(err) => {
                worker::console_error!("Failed to render HTML: {err}");
                Response::InternalServerError()
            }
            Self::Validation(msg) => {
                worker::console_error!("Validation failed: {msg}");
                Response::BadRequest()
            }
            Self::KV(kve) => {
                worker::console_error!("Error from KV: {kve}");
                Response::BadRequest()
            }
        }
    }
}
