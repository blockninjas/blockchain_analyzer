use super::{Input, Output};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct Transaction {
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

impl Transaction {
    pub fn get_input_address_ids(&self) -> Vec<u64> {
        self.inputs
            .iter()
            .filter_map(|input| input.address.to_address_id())
            .collect()
    }

    pub fn get_output_address_ids(&self) -> Vec<u64> {
        self.outputs
            .iter()
            .filter_map(|output| output.address.to_address_id())
            .collect()
    }
}
