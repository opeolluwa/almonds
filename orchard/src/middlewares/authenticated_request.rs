use crate::adapters::jwt::Claims;
use axum::{
    body::Body, extract::{FromRequest, FromRequestParts}, http::{request::Parts, Request}
};

// Newtype to bundle authenticated user with payload
pub struct Authed<T> {
    pub user: Claims,
    pub payload: T,
}

// Custom extractor
impl<S, T> FromRequest<S> for Authed<T>
where
    T: FromRequest<S> + Send,
    Claims: FromRequestParts<S> + Send,
    S: Send + Sync,
{
    type Rejection = T::Rejection;

    async fn from_request(req: axum::http::Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();
        let user: crate::adapters::jwt::Claims = Claims::from_request_parts(&mut &parts, state).await.unwrap(); // You may want to handle this gracefully
        let payload = T::from_request(Request::from_parts(parts, body), state).await?;
        Ok(Self { user, payload })
    }
}
