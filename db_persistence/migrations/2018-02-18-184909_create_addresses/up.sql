CREATE TABLE addresses (
    id SERIAL PRIMARY KEY,
    hash BYTEA NOT NULL,
    base58_string VARCHAR(36) NOT NULL,
    output_id INTEGER NOT NULL REFERENCES outputs (id)
)