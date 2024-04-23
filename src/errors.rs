use ohkami::{Response, IntoResponse};
use worker::send::SendWrapper;


#[allow(unused)]
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
                Response::BadRequest()
            }
            Self::KV(kve) => {
                worker::console_error!("Error from KV: {kve}");
                Response::BadRequest()
            }
            Self::Worker(err) => {
                worker::console_error!("Error in Worker: {err}");
                Response::InternalServerError()
            }
        }
    }
}
