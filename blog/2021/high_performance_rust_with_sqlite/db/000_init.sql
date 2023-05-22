CREATE TABLE IF NOT EXISTS users (
    id BLOB PRIMARY KEY NOT NULL,
    created_at TEXT NOT NULL,
    username TEXT NOT NULL
);

CREATE UNIQUE INDEX idx_users_on_id ON users(id);
