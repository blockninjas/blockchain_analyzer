CREATE TABLE script_witness_items (
    id BIGSERIAL PRIMARY KEY,
    content BYTEA,
    input_id BIGINT NOT NULL REFERENCES inputs (id)
)
