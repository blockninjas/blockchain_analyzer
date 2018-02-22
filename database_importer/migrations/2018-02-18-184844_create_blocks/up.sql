CREATE TABLE blocks (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    hash BINARY(32) NOT NULL,
    version INT UNSIGNED NOT NULL,
    previous_block_hash BINARY(32) NOT NULL,
    merkle_root BINARY(32) NOT NULL,
    creation_time INT UNSIGNED NOT NULL,
    nonce INT UNSIGNED NOT NULL,
    height INT UNSIGNED NOT NULL
)
