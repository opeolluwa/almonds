use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Instant,
};

use axum::{body::Body, http::Request};
use tower::{Layer, Service};

#[derive(Clone)]
pub struct LoggingLayer;

impl<Svc> Layer<Svc> for LoggingLayer {
    type Service = LoggingMiddleware<Svc>;

    fn layer(&self, inner: Svc) -> Self::Service {
        LoggingMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct LoggingMiddleware<Svc> {
    inner: Svc,
}

impl<Svc, ResBody> Service<Request<Body>> for LoggingMiddleware<Svc>
where
    Svc: Service<Request<Body>, Response = axum::http::Response<ResBody>> + Clone + Send + 'static,
    Svc::Future: Send,
    ResBody: Send,
{
    type Response = Svc::Response;
    type Error = Svc::Error;
    type Future = LoggingFuture<Svc::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        // Capture request details before passing to inner service
        let method = req.method().clone();
        let uri = req.uri().clone();
        let start = Instant::now();

        tracing::info!(
            method = %method,
            uri = %uri,
            "Incoming request"
        );

        LoggingFuture {
            future: self.inner.call(req),
            method,
            uri,
            start,
        }
    }
}

pin_project! {
    pub struct LoggingFuture<F> {
        #[pin]
        future: F,
        method: axum::http::Method,
        uri: axum::http::Uri,
        start: Instant,
    }
}

impl<F, ResBody, E> Future for LoggingFuture<F>
where
    F: Future<Output = Result<axum::http::Response<ResBody>, E>>,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match this.future.poll(cx) {
            Poll::Ready(result) => {
                let elapsed = this.start.elapsed();

                match &result {
                    Ok(response) => {
                        tracing::info!(
                            method = %this.method,
                            uri = %this.uri,
                            status = %response.status(),
                            duration_ms = elapsed.as_millis(),
                            "Request completed"
                        );
                    }
                    Err(_) => {
                        tracing::error!(
                            method = %this.method,
                            uri = %this.uri,
                            duration_ms = elapsed.as_millis(),
                            "Request failed"
                        );
                    }
                }

                Poll::Ready(result)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
