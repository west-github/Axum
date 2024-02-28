#![allow(dead_code)]
use serde::Serialize;

#[derive(Debug, Serialize, Default, PartialEq)]
// #[cfg_attr(test, PartialEq)]
pub struct Header<'a> {
    pub aud: Option<&'a str>,

    pub sub: Option<&'a str>,

    pub iss: Option<&'a str>,

    pub tid: Option<&'a str>,

    pub nbf: Option<&'a str>,

    pub iat: Option<&'a str>,

    pub exp: Option<&'a str>,

    /// Footer
    pub ftr: Option<&'a str>,

    /// Implicit assertions
    pub ixa: Option<&'a str>,
}

#[test]
fn test_header() {
    let header = crate::header!("aud" => "https://example.com", "sub" => "http://example.com");

    println!("{:?}", header);
}
