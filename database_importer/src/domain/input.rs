use ::schema::inputs;

#[derive(Insertable)]
#[table_name="inputs"]
pub struct Input {
    pub id: i32,
    pub sequence_number: i32,
    pub previous_tx_hash: Vec<u8>,
    pub previous_tx_output_index: i32,
    pub transaction_id: i32,
}
