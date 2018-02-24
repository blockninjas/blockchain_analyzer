use ::schema::outputs;

#[derive(Insertable)]
#[table_name="outputs"]
pub struct NewOutput {
    pub output_index: i32,
    pub value: i64,
    pub transaction_id: i32,
}
