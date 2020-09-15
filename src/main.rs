use askama::Template;
use tide::{Request, Response, http::mime};
use async_std::fs::read_to_string;
use async_std::path::PathBuf;
use std::convert::TryInto;
use comrak::{markdown_to_html, ComrakOptions};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    name: &'a str,
}

#[derive(Template)]
#[template(path = "docs.html")]
struct DocsTemplate {
    content: String,
}

async fn index_handler(_req: Request<()>) -> Result<Response, tide::Error> {
    let res: Response = IndexTemplate { name: "marisa" }.into();
    Ok(res)
}

async fn docs_handler(req: Request<()>) -> Result<Response, tide::Error> {
    let filename = req.param("path").unwrap_or("index".to_owned());
    let path: PathBuf = [".", format!("{}.md", filename).as_ref()].iter().collect();
    if !path.exists().await {
        return Ok(Response::builder(404).body("Doc not found").build());
    }
    let md = read_to_string(path).await?;
    let html = markdown_to_html(&md, &ComrakOptions::default());
    let template: tide::Body = DocsTemplate {
        content: html,
    }.try_into().unwrap();
    let res = Response::builder(200)
        .body(template)
        .content_type(mime::HTML)
        .build();
    Ok(res)
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let mut app = tide::new();
    app.at("/").get(index_handler);
    app.at("/docs").get(docs_handler);
    app.at("/docs/:path").get(docs_handler);
    app.with(driftwood::DevLogger);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
