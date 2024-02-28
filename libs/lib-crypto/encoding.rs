use super::Result;
use crate::{
    algorithm::Algorithm,
    error::Error,
    header::Header,
    paseto::{encode_paseto_v4_local, verify_paseto_key_len},
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use serde::Serialize;

pub fn b64_encode<T: AsRef<[u8]>>(input: T) -> String {
    URL_SAFE_NO_PAD.encode(input)
}

pub fn b64_encode_part<T: Serialize>(input: &T) -> Result<String> {
    let json = serde_json::to_vec(input).map_err(|_| Error::FailedToEncode)?;
    Ok(b64_encode(json))
}

/// Encode a token to a String
/// ```rust
/// use lib_crypto::encoding::encode;
/// use lib_crypto::header;
/// use lib_crypto::header::Header;
/// let claim = String::from("Some Headers");
/// let key = "wwwwwwwwwwwwwwwlsdippojhbdhijngg";
/// let encode = encode("pv4-local", Some(header!("aud" => "aud")), &claim, key).unwrap();
/// println!("{}", encode);
/// ```
pub fn encode<T: Serialize>(
    alg: &str,
    key: &str,
    claim: T,
    header: Option<Header<'_>>,
) -> Result<String> {
    let input = match alg.parse()? {
        Algorithm::PV4Local => {
            verify_paseto_key_len(key)?;
            encode_paseto_v4_local(key, claim, header)?
        }
        Algorithm::PV4Public | Algorithm::PV3 | Algorithm::PV2 => todo!(),
    };

    Ok(b64_encode(input))
}
