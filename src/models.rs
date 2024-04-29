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
