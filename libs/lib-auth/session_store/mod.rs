pub mod memory;

// #[derive()]
pub struct Session<T> {
    claim: T,
}

#[async_trait::async_trait]
pub trait SessionStore<T> {
    type Error;

    async fn create_session(&self, payload: T) -> Result<(), Self::Error>;

    async fn get_session(&self, token: &str) -> Result<Session<T>, Self::Error>;

    async fn update_session(&self, payload: T) -> Result<Option<T>, Self::Error>;

    async fn delete_session(&self, token: &str) -> Result<(), Self::Error>;
}
