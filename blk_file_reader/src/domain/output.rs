use super::Address;

#[derive(Debug)]
pub struct Output {
    pub index: u32,
    pub value: u64,
    pub addresses: Box<[Address]>,
}
