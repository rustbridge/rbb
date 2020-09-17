use askama::Template;
use tide::{Request, Response};

mod docs;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    name: &'a str,
}

async fn index_handler(_req: Request<()>) -> Result<Response, tide::Error> {
    let res: Response = IndexTemplate { name: "marisa" }.into();
    Ok(res)
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let mut app = tide::new();
    app.at("/").get(index_handler);
    app.at("/docs").get(docs::docs_handler);
    app.at("/docs/:path").get(docs::docs_handler);
    app.at("/static").serve_dir("static/")?;
    app.with(driftwood::DevLogger);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
