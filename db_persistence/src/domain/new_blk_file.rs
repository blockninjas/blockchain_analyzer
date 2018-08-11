use schema::blk_files;

#[derive(Insertable)]
#[table_name = "blk_files"]
pub struct NewBlkFile {
    pub name: String,
    pub number_of_blocks: i32,
}
