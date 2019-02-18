CREATE TABLE address_deduplicator_states (
    id BIGSERIAL PRIMARY KEY,
    output_address_id BIGINT NOT NULL REFERENCES output_addresses (output_id)
)
