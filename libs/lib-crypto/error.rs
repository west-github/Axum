use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    InvalidTokenToEncode,
    InvalidTokenToDecode,

    MissingEncodingClaims,
    MissingDecodingClaims,

    FailedToEncode,
    FailedToDecode,

    UnknownTokenFamily,

    InvalidOffSetTime,

    CustomClaimError,

    InvalidEncryptionKey,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error {}", self)
    }
}
