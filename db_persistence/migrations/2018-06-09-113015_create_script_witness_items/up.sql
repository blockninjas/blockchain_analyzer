CREATE TABLE script_witness_items (
    id BIGSERIAL PRIMARY KEY,
    content BYTEA NOT NULL,
    input_id BIGINT NOT NULL REFERENCES inputs (id)
)
