use blk_file_reader;
use schema::outputs;

#[derive(Insertable)]
#[table_name = "outputs"]
pub struct NewOutput {
    pub output_index: i32,
    pub value: i64,
    pub script: Vec<u8>,
    pub transaction_id: i64,
}

impl NewOutput {
    pub fn new(output: &blk_file_reader::Output, transaction_id: i64) -> NewOutput {
        NewOutput {
            output_index: output.index as i32,
            value: output.value as i64,
            // TODO Avoid copy.
            script: output.script.to_vec(),
            transaction_id,
        }
    }
}
