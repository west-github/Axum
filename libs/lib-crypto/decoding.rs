use super::Result;
use crate::{
    algorithm::Algorithm,
    error::Error,
    header::Header,
    paseto::{decode_paseto_v4_local, verify_paseto_key_len},
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use serde::{de::DeserializeOwned, Deserialize};

#[derive(Debug, Default, Deserialize)]
pub struct DecodeData<T> {
    pub data: T,

    pub aud: Option<String>,

    pub sub: Option<String>,

    pub iss: Option<String>,

    pub tid: Option<String>,

    pub nbf: Option<String>,

    pub iat: Option<String>,

    pub exp: Option<String>,

    pub ftr: Option<String>,

    pub ixa: Option<String>,
}

pub fn b64_decode<T: AsRef<[u8]>>(token: T) -> Result<Vec<u8>> {
    URL_SAFE_NO_PAD
        .decode(token)
        .map_err(|_| Error::FailedToDecode)
}

pub fn b64_decode_part<T: DeserializeOwned>(token: impl AsRef<[u8]>) -> Result<T> {
    let _t = b64_decode(token)?;

    serde_json::from_slice(&_t).map_err(|_| Error::FailedToDecode)
}

pub fn decode<T: DeserializeOwned>(
    token: &str,
    key: &str,
    header: Option<Header<'_>>,
) -> Result<DecodeData<T>> {
    let token = String::from_utf8(b64_decode(token)?).map_err(|_| Error::FailedToDecode)?;

    let _alg = token
        .split(".")
        .collect::<Vec<&str>>()
        .windows(2)
        .next()
        .ok_or(Error::InvalidTokenToDecode)?
        .join(".");

    match _alg.parse()? {
        Algorithm::PV4Local => {
            verify_paseto_key_len(key)?;

            decode_paseto_v4_local(&token, key, header)
        }
        Algorithm::PV4Public | Algorithm::PV3 | Algorithm::PV2 => todo!(),
    }
}
