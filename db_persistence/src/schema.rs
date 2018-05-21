table! {
    addresses (id) {
        id -> Int8,
        base58check -> Varchar,
    }
}

table! {
    blk_files (id) {
        id -> Int8,
        name -> Varchar,
        number_of_blocks -> Int4,
    }
}

table! {
    blocks (id) {
        id -> Int8,
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
        id -> Int8,
        sequence_number -> Int4,
        previous_tx_hash -> Bytea,
        previous_tx_output_index -> Int4,
        script -> Bytea,
        transaction_id -> Int8,
    }
}

table! {
    output_addresses (output_id) {
        output_id -> Int8,
        hash -> Bytea,
        base58check -> Varchar,
    }
}

table! {
    outputs (id) {
        id -> Int8,
        output_index -> Int4,
        value -> Int8,
        script -> Bytea,
        transaction_id -> Int8,
    }
}

table! {
    transactions (id) {
        id -> Int8,
        hash -> Bytea,
        version -> Int4,
        lock_time -> Int4,
        creation_time -> Int4,
        block_id -> Int8,
    }
}

joinable!(inputs -> transactions (transaction_id));
joinable!(output_addresses -> outputs (output_id));
joinable!(outputs -> transactions (transaction_id));
joinable!(transactions -> blocks (block_id));

allow_tables_to_appear_in_same_query!(
    addresses,
    blk_files,
    blocks,
    inputs,
    output_addresses,
    outputs,
    transactions,
);
