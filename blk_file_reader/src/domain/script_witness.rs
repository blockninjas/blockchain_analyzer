pub type ScriptWitnessItem = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScriptWitness {
    pub items: Vec<ScriptWitnessItem>,
}
