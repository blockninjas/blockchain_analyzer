use super::{NewInput, NewOutput};

pub struct NewTransaction {
  pub inputs: Box<[NewInput]>,
  pub outputs: Box<[NewOutput]>,
}
