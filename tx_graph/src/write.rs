use std::io::{Result, Write};
use domain::{InputOutput, NewTransaction};
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
      self.write_input_output(new_input)?;
    }

    for new_output in new_transaction.outputs.iter() {
      self.write_input_output(new_output)?;
    }

    Ok(())
  }
}

trait WriteInputOutput {
  fn write_input_output(&mut self, input_output: &InputOutput) -> Result<()>;
}

impl<W: Write> WriteInputOutput for W {
  fn write_input_output(&mut self, input_output: &InputOutput) -> Result<()> {
    self.write_u64::<LittleEndian>(input_output.value)?;
    self.write_u64::<LittleEndian>(input_output.address_id)?;
    Ok(())
  }
}

#[cfg(test)]
mod test {

  use super::*;
  use domain::memory_layout::size_of_input_output;

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
  fn can_write_input_output() {
    // Given
    let input = InputOutput {
      value: 1,
      address_id: 2,
    };

    // When
    let mut bytes = Vec::<u8>::new();
    bytes.write_input_output(&input).unwrap();

    // Then
    assert_eq!(bytes.len(), size_of_input_output());
  }
}
