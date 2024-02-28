pub mod action;
pub mod error;
pub mod template;

use axum::{
    http::{HeaderName, HeaderValue, StatusCode},
    response::{IntoResponse, Response as AxumResponse},
};
use serde::Serialize;
use serde_with::DisplayFromStr;

// Re export for macro to use
pub use action::Action;
pub use error::Error;

#[doc = r#"Response is always in json format with T passed as the body

```rust
use lib_core::rsp;
use lib_core::action;
use lib_core::OK;
use axum::http::{header::ACCEPT, HeaderValue};

#[derive(serde::Serialize, Copy, Clone)]
pub struct Test {}
let test = Test{};
rsp!(OK, test);
rsp!(OK, action: action!(r: "/"));
rsp!(OK, test, action: action!(a: "Some Message"));

let header_name = ACCEPT;
let header_value = HeaderValue::from_static("true");

rsp!{OK, test, (header_name.clone(), header_value.clone())};
rsp!{OK, action: action!(r: "/"), (header_name.clone(), header_value.clone())};
rsp!{OK, test, action: action!(a: "Some Message"), (header_name, header_value)};
```"#]
#[serde_with::serde_as]
#[cfg_attr(feature = "dev", derive(PartialEq, Debug))]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T = ()> {
    pub body: T,

    #[serde(skip_serializing_if = "Action::is_none")]
    pub action: Action,

    #[serde(skip)]
    pub header: Option<Vec<(HeaderName, HeaderValue)>>,

    #[serde_as(as = "DisplayFromStr")]
    pub status_code: StatusCode,
}

impl<T: Serialize> Response<T> {
    pub fn new(
        status_code: StatusCode,
        body: T,
        action: Action,
        header: Option<Vec<(HeaderName, HeaderValue)>>,
    ) -> Self {
        Self {
            status_code,
            body,
            action,
            header,
        }
    }
}

impl<T: serde::Serialize> IntoResponse for Response<T> {
    fn into_response(mut self) -> AxumResponse {
        let header = self.header.take();

        let mut res = (self.status_code, axum::Json(self)).into_response();

        res.headers_mut()
            .insert("x-json-response", HeaderValue::from_static("true"));

        header.map(|header| res.headers_mut().extend(header));

        res
    }
}
