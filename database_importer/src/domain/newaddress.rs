use ::schema::addresses;

#[derive(Insertable)]
#[table_name="addresses"]
pub struct NewAddress {
    pub value: Vec<u8>,
}
