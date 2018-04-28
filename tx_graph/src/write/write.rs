use std::io::{Result, Write};
use super::{NewInput, NewOutput, NewTransaction};
use byteorder::{LittleEndian, WriteBytesExt};

pub trait WriteTxGraph {
  fn write_transaction(
    &mut self,
    new_transaction: &NewTransaction,
  ) -> Result<()>;

  fn write_input(&mut self, new_input: &NewInput) -> Result<()>;

  fn write_output(&mut self, new_output: &NewOutput) -> Result<()>;
}

impl<W: Write> WriteTxGraph for W {
  fn write_transaction(
    &mut self,
    new_transaction: &NewTransaction,
  ) -> Result<()> {
    // TODO Fix possibly truncating cast.
    self.write_u32::<LittleEndian>(new_transaction.inputs.len() as u32)?;

    // TODO Fix possibly truncating cast.
    self.write_u32::<LittleEndian>(new_transaction.outputs.len() as u32)?;

    for new_input in new_transaction.inputs.iter() {
      self.write_input(new_input)?;
    }

    for new_output in new_transaction.outputs.iter() {
      self.write_output(new_output)?;
    }

    Ok(())
  }

  fn write_input(&mut self, new_input: &NewInput) -> Result<()> {
    self.write_u64::<LittleEndian>(new_input.spent_transaction_id)?;
    self.write_u32::<LittleEndian>(new_input.spent_output_index)?;
    Ok(())
  }

  fn write_output(&mut self, new_output: &NewOutput) -> Result<()> {
    self.write_u64::<LittleEndian>(new_output.spending_transaction_id)?;
    self.write_u32::<LittleEndian>(new_output.spending_input_index)?;
    self.write_u64::<LittleEndian>(new_output.value)?;
    self.write_u64::<LittleEndian>(new_output.address_id)?;
    Ok(())
  }
}
