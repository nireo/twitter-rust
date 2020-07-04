CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "tweet" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    content TEXT UNIQUE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP
);
