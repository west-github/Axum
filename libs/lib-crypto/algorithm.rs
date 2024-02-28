use super::error::Error;
use std::str::FromStr;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Algorithm {
    PV4Local,
    PV4Public,
    PV3,
    PV2,
}

impl FromStr for Algorithm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PV4-LOCAL" | "pv4-local" | "v4.local" => Ok(Algorithm::PV4Local),

            _ => Err(Error::UnknownTokenFamily),
        }
    }
}
// IndigoCoal - Hacker X theme
