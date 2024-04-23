use ohkami::prelude::*;
use crate::{ORIGIN, pages::Layout};


#[derive(Clone)]
pub struct LayoutFang;
impl FangAction for LayoutFang {
    async fn back<'a>(&'a self, res: &'a mut Response) {
        if res.headers.ContentType().is_some_and(|ct| ct.starts_with("text/html")) {
            let content = res.drop_content()
                .map(|bytes| String::from_utf8(bytes.into_owned()).unwrap())
                .unwrap_or_else(String::new);
            *res = Layout { content }.into_response();
        }
    }
}

#[derive(Clone)]
pub struct CSRFang;
impl FangAction for CSRFang {
    async fn fore<'a>(&'a self, req: &'a mut Request) -> Result<(), Response> {
        let origin = req.headers.Origin();
        (origin == Some(ORIGIN))
            .then_some(())
            .ok_or_else(|| {
                worker::console_warn!("Unexpected request from {}", origin.unwrap_or("unkown origin"));
                Response::Forbidden()
            })
    }
}
