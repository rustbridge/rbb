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

#[derive(Debug, Clone)]
pub struct SitemapProvider {
    sitemap: docs::Sitemap,
}

#[tide::utils::async_trait]
impl<T: Clone + Send + Sync + 'static> tide::Middleware<T> for SitemapProvider {
    async fn handle(&self, mut req: Request<T>, next: tide::Next<'_, T>) -> tide::Result {
        req.set_ext(self.sitemap.clone());
        Ok(next.run(req).await)
    }
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let sitemap = docs::construct_sitemap()?;
    println!("Initialized doc sitemap with {} entries", &sitemap.num_entries);
    let mut app = tide::new();
    app.at("/").get(index_handler);
    app.at("/docs").get(docs::docs_handler);
    app.at("/docs/:path").get(docs::docs_handler);
    app.at("/static").serve_dir("static/")?;
    app.with(driftwood::DevLogger);
    app.with(SitemapProvider { sitemap });
    println!("rbb is starting at http://localhost:8080");
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
