use http::{Request, Response};
use std::future::Future;
pub mod auth_basic;

#[async_trait::async_trait]
pub trait Authentication<B> {
    type Body;

    type Response;

    type Future: Future<Output = Result<Request<Self::Body>, Response<Self::Response>>>;

    async fn authorize_session(&mut self, req: Request<B>) -> Self::Future;

    // fn verify_session(&mut self, request: Request)
}
