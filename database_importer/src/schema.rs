table! {
    addresses (id) {
        id -> Int4,
        hash -> Bytea,
        base58_string -> Varchar,
        output_id -> Int4,
    }
}

table! {
    blocks (id) {
        id -> Int4,
        hash -> Bytea,
        version -> Int4,
        previous_block_hash -> Bytea,
        merkle_root -> Bytea,
        creation_time -> Int4,
        nonce -> Int4,
        height -> Nullable<Int4>,
    }
}

table! {
    inputs (id) {
        id -> Int4,
        sequence_number -> Int4,
        previous_tx_hash -> Bytea,
        previous_tx_output_index -> Int4,
        transaction_id -> Int4,
    }
}

table! {
    outputs (id) {
        id -> Int4,
        output_index -> Int4,
        value -> Int8,
        transaction_id -> Int4,
    }
}

table! {
    transactions (id) {
        id -> Int4,
        hash -> Bytea,
        version -> Int4,
        lock_time -> Int4,
        creation_time -> Int4,
        block_id -> Int4,
    }
}

joinable!(addresses -> outputs (output_id));
joinable!(inputs -> transactions (transaction_id));
joinable!(outputs -> transactions (transaction_id));
joinable!(transactions -> blocks (block_id));

allow_tables_to_appear_in_same_query!(
    addresses,
    blocks,
    inputs,
    outputs,
    transactions,
);
