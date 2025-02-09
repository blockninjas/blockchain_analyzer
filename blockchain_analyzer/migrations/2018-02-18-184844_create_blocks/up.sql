CREATE TABLE blocks (
    id BIGSERIAL PRIMARY KEY,
    hash BYTEA NOT NULL,
    version INTEGER NOT NULL,
    previous_block_hash BYTEA NOT NULL,
    merkle_root BYTEA NOT NULL,
    creation_time INTEGER NOT NULL,
    bits INTEGER NOT NULL,
    nonce INTEGER NOT NULL,
    height INTEGER,
    blk_file_id BIGINT NOT NULL REFERENCES blk_files (id)
)
