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

    pub async fn find_messages(&self, after: Option<Uuid>) -> Result<Vec<Message>, Error> {
        let query = "SELECT *
            FROM messages
            WHERE id > $1";

        let messages: Vec<Message> = sqlx::query_as::<_, Message>(query)
            .bind(after.unwrap_or(Uuid::nil()))
            .fetch_all(&self.db)
            .await?;

        Ok(messages)
    }

    pub async fn create_message(&self, body: String) -> Result<Message, Error> {
        if body.len() > 10_000 {
            return Err(Error::InvalidArgument("Message is too large".to_string()));
        }

        let created_at = chrono::Utc::now();
        let id: Uuid = Ulid::new().into();

        let query = "INSERT INTO messages
            (id, created_at, body)
            VALUES ($1, $2, $3)";

        sqlx::query(query)
            .bind(id)
            .bind(created_at)
            .bind(&body)
            .execute(&self.db)
            .await?;

        Ok(Message {
            id,
            created_at,
            body,
        })
    }
}
