table! {
    addresses (id) {
        id -> Integer,
        value -> Nullable<Binary>,
    }
}

table! {
    blocks (id) {
        id -> Integer,
        hash -> Binary,
        version -> Integer,
        previous_block_hash -> Binary,
        merkle_root -> Binary,
        creation_time -> Integer,
        nonce -> Integer,
        height -> Integer,
    }
}

table! {
    inputs (id) {
        id -> Integer,
        sequence_number -> Integer,
        previous_tx_hash -> Binary,
        previous_tx_output_index -> Binary,
        transaction_id -> Integer,
    }
}

table! {
    outputs (id) {
        id -> Integer,
        output_index -> Integer,
        value -> Bigint,
        transaction_id -> Integer,
    }
}

table! {
    transactions (id) {
        id -> Integer,
        hash -> Binary,
        version -> Integer,
        lock_time -> Integer,
        creation_time -> Integer,
        block_id -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    addresses,
    blocks,
    inputs,
    outputs,
    transactions,
);
