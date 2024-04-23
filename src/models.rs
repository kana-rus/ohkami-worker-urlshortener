use ohkami::{Response, IntoResponse, Request, FromRequest};
use ohkami::{typed::Payload, builtin::payload::URLEncoded};
use worker::send::{SendFuture, SendWrapper};
use worker::kv::{KvStore, ToRawKvValue};
use std::{borrow::Cow, future::Future};
use crate::{pages, AppError};


pub use pages::IndexPage;

#[Payload(URLEncoded/D)]
#[derive(Debug)]
pub struct CreateShortenURLForm<'req> {
    pub url: Cow<'req, str>,
}

pub enum CreatedOrErrorPage {
    Created { shorten_url: String },
    Error,
}
impl IntoResponse for CreatedOrErrorPage {
    fn into_response(self) -> Response {
        match self {
            Self::Created { shorten_url } => pages::CreatedPage { shorten_url }.into_response(),
            Self::Error                   => pages::ErrorPage.into_response(),
        }
    }
}

pub struct Host<'req>(&'req str);
impl<'req> FromRequest<'req> for Host<'req> {
    type Error = std::convert::Infallible;
    fn from_request(req: &'req Request) -> Option<Result<Self, Self::Error>> {
        req.headers.Host().map(Self).map(Ok)
    }
}
impl std::fmt::Display for Host<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

pub struct KV(SendWrapper<KvStore>);
impl<'req> FromRequest<'req> for KV {
    type Error = AppError;
    fn from_request(req: &'req Request) -> Option<Result<Self, Self::Error>> {
        Some(req.env().kv("KV").map_err(AppError::Worker)
            .map(|kv| Self(SendWrapper(kv)))
        )
    }
}
impl KV {
    pub fn get<'kv>(&'kv self,
        key: &'kv str,
    ) -> impl Future<Output = Result<Option<String>, AppError>> + Send + 'kv {
        SendFuture::new(async move {
            self.0.get(key.as_ref())
                .text().await
                .map_err(AppError::kv)
        })
    }

    pub fn put<'kv>(&'kv self,
        key:   &'kv str,
        value: impl ToRawKvValue + 'kv,
    ) -> impl Future<Output = Result<(), AppError>> + Send + 'kv {
        SendFuture::new(async move {
            self.0.put(key.as_ref(), value).unwrap()
                .execute().await.map_err(AppError::kv)
        })
    }
}
