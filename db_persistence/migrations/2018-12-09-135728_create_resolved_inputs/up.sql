CREATE VIEW resolved_inputs AS
    SELECT oa.base58check,
        o.value,
        i.id,
        i.sequence_number,
        i.previous_tx_hash,
        i.previous_tx_output_index,
        i.script,
        i.transaction_id
   FROM inputs i
        JOIN transactions tx ON i.previous_tx_hash = tx.hash
        JOIN outputs o ON o.transaction_id = tx.id AND o.output_index = i.previous_tx_output_index
        JOIN output_addresses oa ON oa.output_id = o.id;
