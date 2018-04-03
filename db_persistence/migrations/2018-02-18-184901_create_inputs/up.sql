CREATE TABLE inputs (
    id SERIAL PRIMARY KEY,
    sequence_number INTEGER NOT NULL,
    previous_tx_hash BYTEA NOT NULL,
    previous_tx_output_index INTEGER NOT NULL,
    script BYTEA NOT NULL,
    transaction_id INTEGER NOT NULL REFERENCES transactions (id)
)
