CREATE TABLE transactions (
    id BIGSERIAL PRIMARY KEY,
    hash BYTEA NOT NULL,
    version INTEGER NOT NULL,
    lock_time INTEGER NOT NULL,
    creation_time INTEGER NOT NULL,
    block_id BIGINT NOT NULL REFERENCES blocks (id)
)
