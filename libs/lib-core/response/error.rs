use super::Action;
use axum::{
    http::{HeaderName, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[serde_with::serde_as]
#[cfg_attr(feature = "dev", derive(PartialEq, Debug))]
#[derive(Serialize)]
pub struct Error<T = ()>
where
    T: Serialize,
{
    pub message: String,

    pub help: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<T>,

    #[serde(skip_serializing_if = "Action::is_none")]
    pub action: Action,

    #[serde(skip)]
    pub header: Option<Vec<(HeaderName, HeaderValue)>>,

    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[serde(rename = "statusCode")]
    pub status_code: StatusCode,
}

impl<T: Serialize> Error<T> {
    pub fn new(
        message: String,
        help: String,
        body: Option<T>,
        action: Action,
        header: Option<Vec<(HeaderName, HeaderValue)>>,
        status_code: StatusCode,
    ) -> Self {
        Self {
            message,
            help,
            body,
            action,
            header,
            status_code,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(mut self) -> Response {
        let header = self.header.take();
        let mut res = (self.status_code, axum::Json(self)).into_response();

        res.headers_mut()
            .insert("x-error-response", HeaderValue::from_static("true"));

        header.map(|header| res.headers_mut().extend(header));

        res
    }
}
