use super::super::schema::outputs;

#[derive(Insertable)]
#[table_name="outputs"]
pub struct Output {
    pub id: i32,
    pub output_index: i32,
    pub value: i64,
    pub transaction_id: i32,
}
