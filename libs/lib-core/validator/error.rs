#![allow(unused_variables)]

use axum::{
    extract::rejection::{FormRejection, JsonRejection, QueryRejection},
    response::{IntoResponse, Response},
};
use validator::ValidationErrors;

use crate::{__impl_error_display, __impl_from_for_validator_errors, INTERNAL_SERVER_ERROR};

#[derive(Debug)]
pub enum Error {
    Body(JsonRejection),

    Query(QueryRejection),

    Form(FormRejection),

    Validation(ValidationErrors),
}

__impl_from_for_validator_errors!(JsonRejection, Body);
__impl_from_for_validator_errors!(QueryRejection, Query);
__impl_from_for_validator_errors!(FormRejection, Form);
__impl_from_for_validator_errors!(ValidationErrors, Validation);
__impl_error_display!(Error);

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::Body(_err) => template(),
            Error::Form(_err) => template(),
            Error::Query(_err) => template(),
            Error::Validation(_err) => template(),
        }
    }
}

fn template() -> Response {
    let err = crate::response::Error {
        message: "Internal Server Error Occured".into(),
        help: "500 - Server error occured".into(),
        body: None,
        action: crate::response::Action::NONE,
        header: None,
        status_code: INTERNAL_SERVER_ERROR,
    };

    err.into_response()
}
