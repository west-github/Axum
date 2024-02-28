#![allow(dead_code, unused_variables)]
use error::Error;
mod algorithm;
pub mod decoding;
pub mod encoding;
pub mod error;
pub mod header;
mod macros;
mod paseto;
mod validation;

pub type Result<T> = core::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::{decode, encode};
    use crate::{decoding::DecodeData, header};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Default, Deserialize)]
    pub struct Claim {
        name: String,
        age: i32,
    }

    impl Claim {
        fn new(name: String, age: i32) -> Self {
            Self { name, age }
        }
    }

    #[test]
    fn paseto_v4_encoding_and_decoding_with_claim_and_only() -> super::Result<()> {
        let claim = Claim::new(String::from("West"), 30);
        let key = "NMtg4zh++wURsvw0LCgB9Bcf011KPwWo";

        let _d: DecodeData<Claim> = decode!(&encode!("v4.local", key, claim)?, key)?;

        println!("Test 1: {:#?}", _d);

        Ok(())
    }

    #[test]
    fn paseto_v4_encoding_and_decoding_with_claim_and_header() -> super::Result<()> {
        let claim = Claim::new(String::from("West"), 30);
        let key = "NMtg4zh++wURsvw0LCgB9Bcf011KPwWo";

        let _d: DecodeData<Claim> = decode! {
            &encode!("v4.local", key, claim, header!("aud" => "aud", "sub" => "me"))?,
            key
        }?;

        println!("Test 2: {:#?}", _d);

        Ok(())
    }

    #[test]
    fn paseto_v4_encoding_and_decoding_with_claim_and_header_footer() -> super::Result<()> {
        let claim = Claim::new(String::from("West"), 30);
        let key = "NMtg4zh++wURsvw0LCgB9Bcf011KPwWo";

        let _d: DecodeData<Claim> = decode! {
            &encode!("v4.local", key, claim, header!("aud" => "aud", "sub" => "me", "ftr" => "maybe science is better"))?,
            key,
            header!("ftr" => "maybe science is better")
        }?;

        println!("Test 3: {:#?}", _d);

        Ok(())
    }
}
