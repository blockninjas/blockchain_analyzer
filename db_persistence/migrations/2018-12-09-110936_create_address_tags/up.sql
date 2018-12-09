CREATE TABLE address_tags (
    id BIGSERIAL PRIMARY KEY,
    address_id BIGINT NOT NULL REFERENCES addresses (id),
    title VARCHAR NOT NULL,
    priority SMALLINT NOT NULL DEFAULT 0
);
