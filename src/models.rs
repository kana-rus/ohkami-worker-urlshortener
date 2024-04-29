use ohkami::{Request, FromRequest};
use ohkami::{typed::Payload, builtin::payload::URLEncoded};
use worker::send::{SendFuture, SendWrapper};
use worker::kv::{KvStore, ToRawKvValue};
use std::{borrow::Cow, future::Future};
use crate::{pages, AppError};


pub use pages::IndexPage;

pub use pages::CreatedPage;

#[Payload(URLEncoded/D)]
#[derive(Debug)]
pub struct CreateShortenURLForm<'req> {
    pub url: Cow<'req, str>,
}

// pub struct KV(SendWrapper<KvStore>);
// impl<'req> FromRequest<'req> for KV {
//     type Error = AppError;
//     fn from_request(req: &'req Request) -> Option<Result<Self, Self::Error>> {
//         Some(req.env().kv("KV").map_err(AppError::Worker)
//             .map(|kv| Self(SendWrapper(kv)))
//         )
//     }
// }
// impl KV {
//     pub fn get<'kv>(&'kv self,
//         key: &'kv str,
//     ) -> impl Future<Output = Result<Option<String>, AppError>> + Send + 'kv {
//         SendFuture::new(async move {
//             self.0.get(key)
//                 .text().await
//                 .map_err(AppError::kv)
//         })
//     }
// 
//     pub fn put<'kv>(&'kv self,
//         key:   &'kv str,
//         value: impl ToRawKvValue + 'kv,
//     ) -> impl Future<Output = Result<(), AppError>> + Send + 'kv {
//         SendFuture::new(async move {
//             self.0.put(key, value).unwrap()
//                 .execute().await.map_err(AppError::kv)
//         })
//     }
// }
// 