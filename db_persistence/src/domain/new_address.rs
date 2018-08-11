use schema::addresses;

#[derive(Insertable)]
#[table_name = "addresses"]
pub struct NewAddress {
    pub base58check: String,
}
