CREATE VIEW resolved_senders AS
    SELECT r.base58check AS sender_base58check,
        oa.base58check AS receiver_base58check,
        r.value AS received_value,
        t.id AS transaction_id
    FROM output_addresses oa
        JOIN outputs o ON o.id = oa.output_id
        JOIN transactions t ON t.id = o.transaction_id
        JOIN resolved_inputs r ON r.transaction_id = t.id;
