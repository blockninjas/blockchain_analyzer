pub type WitnessScript = Vec<u8>;

#[derive(Debug, Clone, PartialEq)]
pub struct Witness {
  pub witness_scripts: Vec<WitnessScript>,
}
