use crate::{Error, DB};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    pub fn create_message(body: String) -> Result<(), Error> {
        todo!();
    }
}
