use ::schema::addresses;

#[derive(Insertable)]
#[table_name="addresses"]
pub struct Address {
    pub id: i32,
    pub value: Vec<u8>,
}
