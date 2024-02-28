#[doc = r#"Construct header
aud = AudienceClaim

sub = SubjectClaim

iss = IssuerClaim

tid = TokenIdentificationClaim

nbf = Not Before claim

iat = IssuedAtClaim

exp = ExpirationClaim

ftr = FooterClaim

ixa = Implicit assertion claim
```rust
use lib_crypto::header;
let header = header!("aud" => "aud", "sub" => "sub", "iss" => "iss");
assert_eq!(header, lib_crypto::header::Header{aud: Some("aud"), sub: Some("sub"), iss: Some("iss"), ..Default::default()});
```"#]
#[macro_export]
macro_rules! header {
    ($($ident:expr => $value:expr),+) => {{
        let mut header = $crate::header::Header::default();
        $(
            match $ident {
                "aud" => {header.aud = Some($value)},
                "sub" => {header.sub = Some($value)},
                "iss" => {header.iss = Some($value)},
                "tid" => {header.tid = Some($value)},
                "nbf" => {header.nbf = Some($value)},
                "iat" => {header.iat = Some($value)},
                "exp" => {header.exp = Some($value)},
                "ftr" => {header.ftr = Some($value)},
                "ixa" => {header.ixa = Some($value)},
                _ => todo!(),
            }
        )+

        header
    }};
}

#[doc = r#"
NOTE: Encode with different algorithm paseto v4 supported only now

RETURN: Result<String>


```rust
use lib_crypto::encode;
use lib_crypto::header;
use serde::{Deserialize, Serialize};
use lib_crypto::error::Error;

let key = "";

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

// encoding without header supported
let _: Result<String, Error> = encode!("v4.local", key, Claim::new("West".into(), 30));

// encoding with header and claim supported
let _: Result<String, Error> = encode!("v4.local", key, Claim::new("West".into(), 30), header!("aud" => "aud"));
```
"#]
#[macro_export]
macro_rules! encode {
    ($alg:expr, $key:expr, $claim:expr) => {
        $crate::encoding::encode($alg, $key, $claim, None)
    };

    ($alg:expr, $key:expr, $claim:expr, $header:expr) => {
        $crate::encoding::encode($alg, $key, Some($claim), Some($header))
    };
}

#[doc = r#"
NOTE: T must implement deserialize 

RETURN: Result<DecodeData<T>>

```rust
use lib_crypto::decode;
use lib_crypto::header;
use lib_crypto::error::Error;
use lib_crypto::decoding::DecodeData;
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
// ASSUMPTION: token is encoded with claim 
let key = "";
let token = "";
// SUPPORT: Decoding without header
let res: Result<DecodeData<Claim>, Error> = decode!(token, key);
assert!(res.is_err());

// SUPPORT: Decoding with header
let res: Result<DecodeData<Claim>, Error> = decode!(token, key, header!("aud" => "aud"));
assert!(res.is_err());
```
"#]
#[macro_export]
macro_rules! decode {
    ($token:expr, $key:expr) => {
        $crate::decoding::decode($token, $key, None)
    };

    ($token:expr, $key:expr, $header:expr) => {
        $crate::decoding::decode($token, $key, Some($header))
    };
}
