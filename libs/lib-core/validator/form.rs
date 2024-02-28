use crate::__impl_deref;

use super::error::Error;
use axum::extract::{rejection::FormRejection, Form as axum_form, FromRequest, Request};
use serde::de::DeserializeOwned;
use validator::Validate;

#[derive(Debug, Copy, Clone)]
pub struct Form<T>(pub T);

#[async_trait::async_trait]
impl<T, S> FromRequest<S> for Form<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    axum_form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = Error;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let axum_form(value) = axum_form::<T>::from_request(req, state).await?;

        let _ = value.validate()?;

        Ok(Form(value))
    }
}

__impl_deref!(Form);
