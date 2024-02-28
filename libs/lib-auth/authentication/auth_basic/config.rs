#[derive(Clone)]
pub struct TokenConfig {
    /// Secret key to encode and decode the token
    secret: String,

    /// leeway defined the time the token will expire
    leeway: String,
}

#[derive(Clone)]
pub struct Config {
    /// Token that will be in session
    token: TokenConfig,
}
