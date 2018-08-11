use super::{Input, Output};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct Transaction {
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}
