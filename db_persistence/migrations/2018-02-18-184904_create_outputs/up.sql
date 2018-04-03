CREATE TABLE outputs (
    id SERIAL PRIMARY KEY,
    output_index INTEGER NOT NULL,
    value BIGINT NOT NULL,
    script BYTEA NOT NULL,
    transaction_id INTEGER NOT NULL REFERENCES transactions (id)
)
