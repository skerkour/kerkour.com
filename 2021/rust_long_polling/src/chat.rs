use crate::{Error, DB};
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Message {
    id: Uuid,
    created_at: chrono::DateTime<chrono::Utc>,

    body: String,
}

#[derive(Debug)]
pub struct ChatService {
    db: DB,
}

impl ChatService {
    pub fn new(db: DB) -> Self {
        ChatService { db }
    }

    pub fn find_messages(after: Uuid) -> Result<Vec<Message>, Error> {
        todo!();
    }

    pub async fn create_message(&self, body: String) -> Result<(), Error> {
        if body.len() > 10_000 {
            return Err(Error::InvalidArgument("Message is too large".to_string()));
        }

        let now = chrono::Utc::now();
        let message_id: Uuid = Ulid::new().into();
        let query = "INSERT INTO messages
            (id, created_at, body)
            VALUES ($1, $2, $3)";

        sqlx::query(query)
            .bind(message_id)
            .bind(now)
            .bind(body)
            .execute(&self.db)
            .await?;
        Ok(())
    }
}
