use askama::Template;
use tide::{Request, Response};
use super::common;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    name: &'a str,
    common: common::Common,
}

pub async fn index_handler(_req: Request<()>) -> Result<Response, tide::Error> {
    let res: Response = IndexTemplate { name: "RustBridge Berlin", common: common::gen_common() }.into();
    Ok(res)
}