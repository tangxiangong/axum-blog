use std::sync::Arc;
use tower_sessions::{session::Id, Expiry, MemoryStore, Session as SessionCore, SessionStore};

use crate::{AppError, AppResult};

#[derive(Debug)]
pub struct Session(SessionCore);

impl Default for Session {
    fn default() -> Self {
        let id = Id::default();
        let store = Arc::new(MemoryStore::default());
        let expiry = Expiry::OnSessionEnd;
        let session = SessionCore::new(Some(id), store, Some(expiry));
        Self(session)
    }
}

impl Session {
    const SESSION_KEY: &'static str = "username";
    pub fn new(store: Arc<impl SessionStore>, expiry: Expiry) -> Self {
        let id = Id::default();
        let session = SessionCore::new(Some(id), store, Some(expiry));
        Self(session)
    }

    pub async fn insert(&self, uername: impl Into<String>) -> AppResult {
        let username: String = uername.into();
        self.0
            .insert(Self::SESSION_KEY, username)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;
        Ok(())
    }

    pub async fn get(&self) -> AppResult<Option<String>> {
        let username = self
            .0
            .get::<String>(Self::SESSION_KEY)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;
        Ok(username)
    }

    pub fn id(&self) -> Option<Id> {
        self.0.id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_session() {
        let session = Session::default();
        session.insert("username").await.unwrap();
        println!("session id: {}", session.id().unwrap());
        let username = session.get().await.unwrap().unwrap();
        assert_eq!(username, "username");
    }
}
