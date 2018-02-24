use ::schema::outputs;
use blk_file_reader;

#[derive(Insertable)]
#[table_name="outputs"]
pub struct NewOutput {
    pub output_index: i32,
    pub value: i64,
    pub transaction_id: i32,
}

impl NewOutput {
    pub fn new(output: &blk_file_reader::Output, transaction_id: i32) -> NewOutput {
        NewOutput {
            output_index: output.index as i32,
            value: output.value as i64,
            transaction_id,
        }
    }
}
