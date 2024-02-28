use crate::{
    authentication::Authentication,
    future::ResponseFuture,
    session_store::{memory::MemoryStore, SessionStore},
};
use http::{Request, Response};
use std::task::{Context, Poll};
use tower_service::Service;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Session<L, A, S> {
    pub(crate) inner: L,
    pub(crate) auth: A,
    pub(crate) store: S,
}

impl<L, A, S> Session<L, A, S> {
    pub fn create_session(&self) -> Result<(), ()> {
        Ok(())
    }

    pub fn revoke_session(&self) -> Result<(), ()> {
        Ok(())
    }

    pub fn delete_session(&self) -> Result<(), ()> {
        Ok(())
    }
}

pub struct Claim {}

impl<ReqBody, ResBody, L, A, S> Service<Request<ReqBody>> for Session<L, A, S>
where
    A: Authentication<ReqBody>,
    S: SessionStore<Claim>,
    L: Service<Request<ReqBody>, Response = Response<ResBody>>,
{
    type Response = Response<ResBody>;
    type Error = L::Error;
    type Future = ResponseFuture<L::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        ResponseFuture {
            future: self.inner.call(req),
        }
    }
}
