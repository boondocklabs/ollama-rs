use http::Extensions;
use reqwest::{Request, Response};
use reqwest_middleware::{Middleware, Next};
use tracing::debug;

pub(crate) struct LoggingMiddleware;

#[async_trait::async_trait]
impl Middleware for LoggingMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> reqwest_middleware::Result<Response> {
        debug!("Request started {:?}", req);
        let res = next.run(req, extensions).await;
        debug!("Result: {:?}", res);
        res
    }
}
