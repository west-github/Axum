use crate::__impl_deref;

use super::error::Error;
use axum::{
    extract::{FromRequestParts, Query as axum_query},
    http::request::Parts,
};
use serde::de::DeserializeOwned;
use validator::Validate;

#[derive(Debug, Copy, Clone)]
pub struct Query<T>(pub T);

#[async_trait::async_trait]
impl<T, S> FromRequestParts<S> for Query<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let axum_query(value) = axum_query::<T>::try_from_uri(&parts.uri)?;

        let _ = value.validate()?;

        Ok(Query(value))
    }
}

__impl_deref!(Query);
