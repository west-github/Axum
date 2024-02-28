#![allow(dead_code, unused_variables)]
use super::{Session, SessionStore};
use http::StatusCode;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct MemoryStore<T> {
    session: Arc<Mutex<Vec<T>>>,
}

impl<T> Clone for MemoryStore<T> {
    fn clone(&self) -> Self {
        let session = self.session.clone();
        Self { session }
    }
}

#[async_trait::async_trait]
impl<T> SessionStore<T> for MemoryStore<T>
where
    T: Send,
{
    type Error = (StatusCode, String);

    async fn create_session(&self, payload: T) -> Result<(), Self::Error> {
        todo!()
    }

    async fn get_session(&self, token: &str) -> Result<Session<T>, Self::Error> {
        todo!()
    }

    async fn update_session(&self, payload: T) -> Result<Option<T>, Self::Error> {
        todo!()
    }

    async fn delete_session(&self, token: &str) -> Result<(), Self::Error> {
        todo!()
    }
}
