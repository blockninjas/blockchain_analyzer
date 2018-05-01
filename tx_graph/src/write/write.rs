use std::io::{Result, Write};
use super::{NewInput, NewOutput, NewTransaction};
use byteorder::{LittleEndian, WriteBytesExt};

pub trait WriteTransaction {
  fn write_transaction(
    &mut self,
    new_transaction: &NewTransaction,
  ) -> Result<()>;
}

impl<W: Write> WriteTransaction for W {
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
}

trait WriteInput {
  fn write_input(&mut self, new_input: &NewInput) -> Result<()>;
}

impl<W: Write> WriteInput for W {
  fn write_input(&mut self, new_input: &NewInput) -> Result<()> {
    self.write_u64::<LittleEndian>(new_input.spent_transaction_id)?;
    self.write_u32::<LittleEndian>(new_input.spent_output_index)?;
    Ok(())
  }
}

trait WriteOutput {
  fn write_output(&mut self, new_output: &NewOutput) -> Result<()>;
}

impl<W: Write> WriteOutput for W {
  fn write_output(&mut self, new_output: &NewOutput) -> Result<()> {
    self.write_u64::<LittleEndian>(new_output.spending_transaction_id)?;
    self.write_u32::<LittleEndian>(new_output.spending_input_index)?;
    self.write_u64::<LittleEndian>(new_output.value)?;
    self.write_u64::<LittleEndian>(new_output.address_id)?;
    Ok(())
  }
}

#[cfg(test)]
mod test {

  use super::*;

  #[test]
  fn can_write_empty_transaction() {
    // Given
    let transaction = NewTransaction {
      inputs: vec![].into_boxed_slice(),
      outputs: vec![].into_boxed_slice(),
    };

    // When
    let mut bytes = Vec::<u8>::new();
    bytes.write_transaction(&transaction).unwrap();

    // Then
    assert_eq!(bytes, [0u8; 8]);
  }

  #[test]
  fn can_write_input() {
    // Given
    let input = NewInput {
      spent_transaction_id: 1,
      spent_output_index: 2,
    };

    // When
    let mut bytes = Vec::<u8>::new();
    bytes.write_input(&input).unwrap();

    // Then
    assert_eq!(bytes.len(), 12);
  }

  #[test]
  fn can_write_output() {
    // Given
    let output = NewOutput {
      spending_transaction_id: 1,
      spending_input_index: 2,
      value: 3,
      address_id: 4,
    };

    // When
    let mut bytes = Vec::<u8>::new();
    bytes.write_output(&output).unwrap();

    // Then
    assert_eq!(bytes.len(), 28);
  }
}
