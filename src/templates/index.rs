use askama::Template;
use tide::{Request, Response};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    name: &'a str,
}

pub async fn index_handler(_req: Request<()>) -> Result<Response, tide::Error> {
    let res: Response = IndexTemplate { name: "marisa" }.into();
    Ok(res)
}