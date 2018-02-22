CREATE TABLE inputs (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    sequence_number INT UNSIGNED NOT NULL,
    previous_tx_hash BINARY(32) NOT NULL,
    previous_tx_output_index BINARY(32) NOT NULL,
    transaction_id INT UNSIGNED NOT NULL REFERENCES transactions (id)
)
