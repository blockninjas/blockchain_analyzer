CREATE TABLE addresses (
    id BIGSERIAL PRIMARY KEY,
    base58check VARCHAR(36) NOT NULL,
    cluster_representative BIGINT REFERENCES addresses (id)
)
