use http::Response;
use pin_project_lite::pin_project;
use std::{
    future::Future,
    pin::Pin,
    task::{ready, Context, Poll},
};

pin_project! {
    #[derive(Debug)]
    pub struct ResponseFuture<F> {
        #[pin]
        pub(crate) future: F,
    }
}

impl<F, B, E> Future for ResponseFuture<F>
where
    F: Future<Output = Result<Response<B>, E>>,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("This is the context after request");

        let this = self.project();

        // Response
        let res = ready!(this.future.poll(cx)?);

        Poll::Ready(Ok(res))
    }
}
