use super::Result;
use crate::{decoding::DecodeData, error::Error, header::Header};
use rusty_paseto::{
    generic::{
        AudienceClaim, CustomClaim, ExpirationClaim, Footer, ImplicitAssertion, IssuedAtClaim,
        IssuerClaim, Key, Local, NotBeforeClaim, PasetoSymmetricKey, SubjectClaim,
        TokenIdentifierClaim, V4,
    },
    prelude::{PasetoBuilder, PasetoParser},
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

pub(crate) fn encode_paseto_v4_local<T: Serialize>(
    key: &str,
    claim: T,
    header: Option<Header>,
) -> Result<String> {
    let mut t = PasetoBuilder::<V4, Local>::default();

    if let Some(_h) = header {
        _h.aud.map(|_v| t.set_claim(AudienceClaim::from(_v)));
        _h.sub.map(|_v| t.set_claim(SubjectClaim::from(_v)));
        _h.iss.map(|_v| t.set_claim(IssuerClaim::from(_v)));
        _h.tid.map(|_v| t.set_claim(TokenIdentifierClaim::from(_v)));

        if let Some(n) = _h.nbf {
            t.set_claim(NotBeforeClaim::try_from(n).map_err(|_| Error::InvalidOffSetTime)?);
        }

        if let Some(i) = _h.iat {
            t.set_claim(IssuedAtClaim::try_from(i).map_err(|_| Error::InvalidOffSetTime)?);
        }

        if let Some(e) = _h.exp {
            t.set_claim(ExpirationClaim::try_from(e).map_err(|_| Error::InvalidOffSetTime)?);
        }

        _h.ftr.map(|_v| t.set_footer(Footer::from(_v)));
        _h.ixa
            .map(|_v| t.set_implicit_assertion(ImplicitAssertion::from(_v)));
    }

    let cc_to_str = serde_json::to_string(&claim).map_err(|_| Error::FailedToEncode)?;
    let cc = CustomClaim::try_from(("data", cc_to_str)).map_err(|_| Error::CustomClaimError)?;
    t.set_claim(cc);

    let key = PasetoSymmetricKey::<V4, Local>::from(Key::from(key.as_bytes()));
    Ok(t.build(&key).map_err(|_| Error::FailedToEncode)?)
}

pub(crate) fn decode_paseto_v4_local<T: DeserializeOwned>(
    token: &str,
    key: &str,
    header: Option<Header>,
) -> Result<DecodeData<T>> {
    let key = PasetoSymmetricKey::<V4, Local>::from(Key::<32>::from(key.as_bytes()));
    let mut p = PasetoParser::<V4, Local>::default();

    header.map(|Header { ftr, ixa, .. }| {
        ftr.map(|f| p.set_footer(Footer::from(f)));
        ixa.map(|i| p.set_implicit_assertion(ImplicitAssertion::from(i)));
    });

    let value = p.parse(token, &key).map_err(|_| Error::FailedToDecode)?;

    fn from_value(value: &Value, key: &str) -> Option<String> {
        value.get(key)?.as_str().map(String::from)
    }

    Ok(DecodeData {
        data: serde_json::from_str(&from_value(&value, "data").ok_or(Error::FailedToDecode)?)
            .map_err(|_| Error::FailedToDecode)?,
        aud: from_value(&value, "aud"),
        sub: from_value(&value, "sub"),
        iss: from_value(&value, "iss"),
        tid: from_value(&value, "tid"),
        nbf: from_value(&value, "nbf"),
        iat: from_value(&value, "iat"),
        exp: from_value(&value, "exp"),
        ftr: from_value(&value, "ftr"),
        ixa: from_value(&value, "ixa"),
    })
}

pub fn verify_paseto_key_len(key: &str) -> Result<()> {
    const PASETO_KEY_LENGTH: usize = 32;

    (key.len() == PASETO_KEY_LENGTH)
        .then_some(())
        .ok_or(Error::InvalidEncryptionKey)
}
