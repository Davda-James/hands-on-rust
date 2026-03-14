ALTER TABLE users ADD COLUMN password TEXT;
ALTER TABLE users ADD COLUMN email TEXT;

CREATE TABLE refresh_tokens (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    token TEXT NOT NULL,
    expires_at TIMESTAMP NOT NULL
);
