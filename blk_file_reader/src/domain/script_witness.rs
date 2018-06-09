pub type ScriptWitnessItem = Vec<u8>;

#[derive(Debug, Clone, PartialEq)]
pub struct ScriptWitness {
  pub items: Vec<ScriptWitnessItem>,
}
