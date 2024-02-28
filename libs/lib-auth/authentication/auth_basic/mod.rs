use std::{future::Future, pin::Pin};

use http::{Request, Response};

use crate::authentication::Authentication;

mod config;

#[derive(Clone)]
pub struct TokenBasedAuthentication {
    // config: Config,
}

#[async_trait::async_trait]
impl<B> Authentication<B> for TokenBasedAuthentication
where
    B: Send + 'static,
{
    type Body = B;

    type Response = Response<()>;

    type Future =
        Pin<Box<dyn Future<Output = Result<Request<Self::Body>, Response<Self::Response>>>>>;

    async fn authorize_session(&mut self, req: http::Request<B>) -> Self::Future {
        todo!()
    }
}
