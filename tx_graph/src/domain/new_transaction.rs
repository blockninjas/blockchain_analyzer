use super::InputOutput;

pub struct NewTransaction {
  pub inputs: Box<[InputOutput]>,
  pub outputs: Box<[InputOutput]>,
}
