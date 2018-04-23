CREATE TABLE addresses (
    id BIGSERIAL PRIMARY KEY,
    hash BYTEA NOT NULL,
    base58check VARCHAR(36) NOT NULL,
    output_id BIGINT NOT NULL REFERENCES outputs (id)
)
