use askama::Template;
use tide::{Response, Request, http::mime};
use comrak::{markdown_to_html, ComrakOptions};
use async_std::fs::read_to_string;
use async_std::path::PathBuf;
use std::convert::TryInto;

#[derive(Template)]
#[template(path = "docs.html")]
struct DocsTemplate {
    content: String,
}

pub async fn docs_handler(req: Request<()>) -> Result<Response, tide::Error> {
    let filename = req.param("path").unwrap_or("index".to_owned());
    let path: PathBuf = ["docs", format!("{}.md", filename).as_ref()].iter().collect();
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
