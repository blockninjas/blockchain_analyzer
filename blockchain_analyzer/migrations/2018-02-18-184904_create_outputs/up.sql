CREATE TABLE outputs (
    id BIGSERIAL PRIMARY KEY,
    output_index INTEGER NOT NULL,
    value BIGINT NOT NULL,
    script BYTEA NOT NULL,
    transaction_id BIGINT NOT NULL REFERENCES transactions (id)
)
