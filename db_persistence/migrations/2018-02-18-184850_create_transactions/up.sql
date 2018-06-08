CREATE TABLE transactions (
    id BIGSERIAL PRIMARY KEY,
    hash BYTEA NOT NULL,
    version INTEGER NOT NULL,
    lock_time INTEGER NOT NULL,
    size_in_bytes INTEGER NOT NULL,
    block_id BIGINT NOT NULL REFERENCES blocks (id)
)
