CREATE TABLE output_addresses (
    output_id BIGINT PRIMARY KEY REFERENCES outputs (id),
    hash BYTEA NOT NULL,
    base58check VARCHAR(36) NOT NULL
)
