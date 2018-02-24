CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    hash BYTEA NOT NULL,
    version INTEGER NOT NULL,
    lock_time INTEGER NOT NULL,
    creation_time INTEGER NOT NULL,
    block_id INTEGER NOT NULL REFERENCES blocks (id)
)
