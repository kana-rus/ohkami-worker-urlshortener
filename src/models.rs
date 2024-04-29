use ohkami::{typed::Payload, builtin::payload::URLEncoded};
use std::borrow::Cow;
use crate::pages;


pub use pages::IndexPage;

pub use pages::CreatedPage;

#[Payload(URLEncoded/D)]
#[derive(Debug)]
pub struct CreateShortenURLForm<'req> {
    pub url: Cow<'req, str>,
}

pub struct Worker<'w>(night_worker::Worker<'w>);
impl<'req> ohkami::FromRequest<'req> for Worker<'req> {
    type Error = std::convert::Infallible;
    fn from_request(req: &'req ohkami::prelude::Request) -> Option<Result<Self, Self::Error>> {
        Some(Ok(Self(
            night_worker::Worker::take_over(req.env(), req.context())
        )))
    }
}
impl<'w> std::ops::Deref for Worker<'w> {
    type Target = night_worker::Worker<'w>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
