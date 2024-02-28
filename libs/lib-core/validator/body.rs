use super::error::Error;
use crate::__impl_deref;
use axum::extract::{rejection::JsonRejection, FromRequest, Json, Request};
use serde::de::DeserializeOwned;
use validator::Validate;

#[derive(Debug, Copy, Clone)]
pub struct Body<T>(pub T);

#[async_trait::async_trait]
impl<T, S> FromRequest<S> for Body<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = Error;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::from_request(req, state).await?;

        let _ = value.validate()?;

        Ok(Body(value))
    }
}

__impl_deref!(Body);
