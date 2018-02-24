use ::schema::addresses;

#[derive(Insertable)]
#[table_name="addresses"]
pub struct NewAddress {
    pub hash: Vec<u8>,
    pub base58_string: String,
}
