use askama::Template;
use tide::{Response, Request, http::mime};
use comrak::{markdown_to_html, ComrakOptions};
use async_std::fs::read_to_string;
use async_std::path::PathBuf;
use walkdir::WalkDir;
use std::convert::TryInto;

#[derive(Template)]
#[template(path = "docs.html")]
struct DocsTemplate<'a> {
    content: String,
    sitemap: &'a Sitemap,

}

#[derive(Debug, Clone)]
pub struct Sitemap {
    pub num_entries: usize,
    pub entries: Vec<String>,
}

pub async fn docs_handler(req: Request<()>) -> Result<Response, tide::Error> {
    let sitemap = req.ext::<Sitemap>().unwrap();
    let filename = req.param("path").unwrap_or("index");
    let path: PathBuf = ["docs", format!("{}.md", filename).as_ref()].iter().collect();
    if !path.exists().await {
        return Ok(Response::builder(404).body("Doc not found").build());
    }
    let md = read_to_string(path).await?;
    let html = markdown_to_html(&md, &ComrakOptions::default());
    let template: tide::Body = DocsTemplate {
        content: html,
        sitemap,
    }.try_into().unwrap();
    let res = Response::builder(200)
        .body(template)
        .content_type(mime::HTML)
        .build();
    Ok(res)
}

pub fn construct_sitemap() -> Result<Sitemap, std::io::Error> {
    let mut res = Vec::new();
    let iter = WalkDir::new("docs/")
        .into_iter()
        .filter(|e| !e.as_ref().ok().unwrap().file_type().is_dir());
    for entry in iter {
        let entry = entry?;
        let mut path = entry.path().to_path_buf();
        path.set_extension("html");
        res.push(path.into_os_string().into_string().unwrap());
    }
    Ok(Sitemap {
        num_entries: res.len(),
        entries: res,
    })
}
