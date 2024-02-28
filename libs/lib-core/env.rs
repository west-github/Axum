use std::{env, str::FromStr};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Missing Environment variables")]
    MissingEnv(&'static str),

    #[error("Wrong Format")]
    WrongFormat(&'static str),
}

pub type Result<T> = core::result::Result<T, Error>;

pub fn get_env(name: &'static str) -> Result<&'static str> {
    let res = env::var(name).map_err(|_| Error::MissingEnv(name))?;

    Ok(Box::leak(res.into_boxed_str()))
}

pub fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
    let value = get_env(name)?;

    value.parse::<T>().map_err(|_| Error::WrongFormat(name))
}
