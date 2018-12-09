CREATE VIEW resolved_receivers AS
    SELECT r.base58check AS sender_base58check,
        oa.base58check AS receiver_base58check,
        o.value AS sent_value,
        r.transaction_id
    FROM resolved_inputs r
         JOIN transactions t ON t.id = r.transaction_id
         JOIN outputs o ON r.transaction_id = o.transaction_id
         JOIN output_addresses oa ON oa.output_id = o.id;
