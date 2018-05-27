use byteorder::{LittleEndian, ReadBytesExt};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use domain::*;
use keys;
use script;
use script::Script;
use std::io::{Cursor, Error, ErrorKind, Read, Result};

/// The magic number which identifies blocks in bitcoin's main network.
const MAIN_NET_MAGIC_NUMBER: u32 = 0xD9B4BEF9;

/// This trait allows for reading `Block`s from blk files.
pub trait ReadBlock: Read {
  /// Read a `Block` from the underlying blk file.
  ///
  /// For more information on the structure of blocks within a blk file refer
  /// to the [according wiki entry](https://en.bitcoin.it/wiki/Block).
  fn read_block(&mut self) -> Result<Block>;
}

/// Internal helper trait.
trait ReadBlockInternals: Read {
  /// Read `Transactions` of a `Block` from the underlying blk file.
  fn read_transactions(&mut self) -> Result<Box<[Transaction]>>;

  /// Read a `Transaction` from the underlying blk file.
  ///
  /// For more information on the structure of transactions within a blk file
  /// refer to the [according wiki entry](https://en.bitcoin.it/wiki/Transaction).
  fn read_transaction(&mut self) -> Result<Transaction>;

  /// Read `Inputs` of a `Transaction` from the underlying blk file.
  fn read_inputs(&mut self, input_count: u32) -> Result<Box<[Input]>>;

  /// Read an `Input` from the underlying blk file.
  ///
  /// For more information on the structure of transactions within a blk file
  /// refer to the [according wiki entry](https://en.bitcoin.it/wiki/Transaction#General_format_.28inside_a_block.29_of_each_input_of_a_transaction_-_Txin).
  fn read_input(&mut self) -> Result<Input>;

  /// Read `Outputs` of a `Transaction` from the underlying blk file.
  fn read_outputs(&mut self, output_count: u32) -> Result<Box<[Output]>>;

  /// Read an `Output` from the underlying blk file.
  ///
  /// For more information on the structure of transactions within a blk file
  /// refer to the [according wiki entry](https://en.bitcoin.it/wiki/Transaction#General_format_.28inside_a_block.29_of_each_output_of_a_transaction_-_Txout).
  fn read_output(&mut self, index: u32) -> Result<Output>;

  /// Read a 256-bit `Hash` from the underyling blk file.
  fn read_hash(&mut self) -> Result<Hash>;

  /// Read a variable-length integer from the underyling blk file.
  ///
  /// For more information on the structure of variable-length integers within a
  /// blk file refer to the [according wiki entry](https://en.bitcoin.it/wiki/Protocol_documentation#Variable_length_integer).
  fn read_var_int(&mut self) -> Result<u64>;

  /// Read a bitcoin script from the underlying blk file.
  fn read_script(&mut self) -> Result<Box<[u8]>>;
}

/// Implement `ReadBlock` for all types that implement `Read`.
impl<R: Read + ?Sized> ReadBlock for R {
  fn read_block(&mut self) -> Result<Block> {
    let magic_number = self.read_u32::<LittleEndian>()?;

    if magic_number != MAIN_NET_MAGIC_NUMBER {
      return Err(Error::new(
        ErrorKind::Other,
        "invalid magic number",
      ));
    }

    let block_size = self.read_u32::<LittleEndian>()?;

    // TODO Fix possibly truncating cast.
    let mut block_content = Box::<[u8]>::from(vec![0u8; block_size as usize]);
    self.read_exact(&mut block_content)?;

    let mut block_content_reader = Cursor::new(block_content);

    let mut block_header = [0u8; 80];
    block_content_reader.read_exact(&mut block_header)?;

    let hash = calculate_hash(&block_header)?;

    let mut block_header_reader = Cursor::new(&block_header[..]);
    let version = block_header_reader.read_u32::<LittleEndian>()?;
    let previous_block_hash = block_header_reader.read_hash()?;
    let merkle_root = block_header_reader.read_hash()?;
    let creation_time = block_header_reader.read_u32::<LittleEndian>()?;
    let bits = block_header_reader.read_u32::<LittleEndian>()?;
    let nonce = block_header_reader.read_u32::<LittleEndian>()?;

    let transactions = block_content_reader.read_transactions()?;

    let block = Block {
      creation_time,
      hash,
      merkle_root,
      bits,
      nonce,
      previous_block_hash,
      version,
      transactions,
    };

    Ok(block)
  }
}

/// Implement `ReadBlock` for `Cursor`s over byte arrays.
impl<B: AsRef<[u8]>> ReadBlockInternals for Cursor<B> {
  fn read_transactions(&mut self) -> Result<Box<[Transaction]>> {
    let transaction_count = self.read_var_int()?;
    // TODO Fix possibly truncating cast.
    let mut transactions = Vec::with_capacity(transaction_count as usize);
    for _ in 0..transaction_count {
      let transaction = self.read_transaction()?;
      transactions.push(transaction);
    }
    Ok(transactions.into_boxed_slice())
  }

  fn read_transaction(&mut self) -> Result<Transaction> {
    let start_position = self.position();

    let version = self.read_u32::<LittleEndian>()?;

    // TODO Fix possibly truncating cast.
    let input_count = self.read_var_int()? as u32;

    let mut inputs: Box<[Input]> = Box::new([]);
    let mut outputs: Box<[Output]> = Box::new([]);

    if input_count == 0 {
      let flags = self.read_u8()?;
      if flags != 0 {
        // TODO Fix possibly truncating cast.
        let input_count = self.read_var_int()? as u32;
        inputs = self.read_inputs(input_count)?;
        // TODO Fix possibly truncating cast.
        let output_count = self.read_var_int()? as u32;
        outputs = self.read_outputs(output_count)?;

        if (flags & 1u8) == 1u8 {
          for _ in 0..input_count {
            let stack_item_count = self.read_var_int()?;
            for _ in 0..stack_item_count {
              let stack_length = self.read_var_int()?;
              // TODO Fix possibly truncating cast.
              let mut stack_item =
                Box::<[u8]>::from(vec![0u8; stack_length as usize]);
              self.read_exact(&mut stack_item)?;
              // TODO How to interpret the stack script?
            }
          }
        }
      }
    } else {
      inputs = self.read_inputs(input_count)?;
      // TODO Fix possibly truncating cast.
      let output_count = self.read_var_int()? as u32;
      outputs = self.read_outputs(output_count)?;
    }

    let lock_time = self.read_u32::<LittleEndian>()?;

    // Calculate the length of the raw transaction data.
    let end_position = self.position();
    let tx_length = end_position - start_position;
    assert_ne!(tx_length, 0);

    // Get the raw transaction data.
    self.set_position(start_position);
    let mut tx_content = Box::<[u8]>::from(vec![0u8; tx_length as usize]);
    self.read_exact(&mut tx_content)?;
    assert_eq!(self.position(), end_position);

    // Calculate the transaction hash over the raw transaction data.
    let tx_hash = calculate_hash(&tx_content)?;

    let transaction = Transaction {
      tx_hash,
      version,
      lock_time,
      inputs,
      outputs,
      block_height: 0,
    };

    Ok(transaction)
  }

  fn read_inputs(&mut self, input_count: u32) -> Result<Box<[Input]>> {
    // TODO Fix possibly truncating cast.
    let mut inputs = Vec::with_capacity(input_count as usize);
    for _ in 0..input_count {
      let input = self.read_input()?;
      inputs.push(input);
    }
    Ok(inputs.into_boxed_slice())
  }

  fn read_input(&mut self) -> Result<Input> {
    let previous_tx_hash = self.read_hash()?;
    let previous_tx_output_index = self.read_u32::<LittleEndian>()?;
    let script = self.read_script()?;
    let sequence_number = self.read_u32::<LittleEndian>()?;

    let input = Input {
      sequence_number,
      previous_tx_hash,
      previous_tx_output_index,
      script,
    };

    Ok(input)
  }

  fn read_outputs(&mut self, output_count: u32) -> Result<Box<[Output]>> {
    // TODO Fix possibly truncating cast.
    let mut outputs = Vec::with_capacity(output_count as usize);
    for output_index in 0..output_count {
      let output = self.read_output(output_index)?;
      outputs.push(output);
    }
    Ok(outputs.into_boxed_slice())
  }

  fn read_output(&mut self, index: u32) -> Result<Output> {
    let value = self.read_u64::<LittleEndian>()?;
    let script = self.read_script()?;
    // TODO Avoid copy.
    let address = read_output_address(script.to_vec());

    let output = Output {
      index,
      value,
      address,
      script,
    };

    Ok(output)
  }

  fn read_hash(&mut self) -> Result<Hash> {
    let mut hash: [u8; 32] = [0; 32];
    self.read_exact(&mut hash)?;
    hash.reverse();
    Ok(Hash(hash))
  }

  fn read_var_int(&mut self) -> Result<u64> {
    let mut control_byte: [u8; 1] = [0];
    self.read_exact(&mut control_byte)?;

    let var_int: u64 = if control_byte[0] < 0xFDu8 {
      control_byte[0] as u64
    } else if control_byte[0] == 0xFDu8 {
      self.read_u16::<LittleEndian>()? as u64
    } else if control_byte[0] == 0xFEu8 {
      self.read_u32::<LittleEndian>()? as u64
    } else {
      self.read_u64::<LittleEndian>()?
    };

    Ok(var_int)
  }

  fn read_script(&mut self) -> Result<Box<[u8]>> {
    let script_length = self.read_var_int()?;
    let mut script = Box::<[u8]>::from(vec![0u8; script_length as usize]);
    self.read_exact(&mut script)?;
    Ok(script)
  }
}

/// Read the receiver address that is contained in the given output script.
///
/// Returns the address on success or `None` if the structure of the script does
/// not conform to any known "standard" script-type.
fn read_output_address(script: Vec<u8>) -> Option<Address> {
  let script = Script::from(script);
  // TODO Return a meaningful error instead of panicking.
  let script_addresses = script
    .extract_destinations()
    .expect("Invalid addresses");

  if script_addresses.len() == 1 {
    let script_address = &script_addresses[0];
    let base58check = base58check_encode(script_address);
    let hash = script_address.hash.clone().take();
    let address = Address {
      hash,
      base58check,
    };
    Some(address)
  } else {
    None
  }
}

fn calculate_hash(bytes: &[u8]) -> Result<Hash> {
  let mut sha = Sha256::new();

  // first hash round
  sha.input(&bytes);
  let mut first_hash = [0u8; 32];
  sha.result(&mut first_hash);

  // second hash round
  sha.reset();
  sha.input(&first_hash);
  let mut second_hash = [0u8; 32];
  sha.result(&mut second_hash);

  second_hash.reverse();

  Ok(Hash(second_hash))
}

/// Transforms the given `ScriptAddress` to a `keys::Address`.
///
/// This is done by leveraging the `Format` trait implementation of
/// `keys::Address` to retrieve it as base58check-encoded string.
// TODO Investigate more elegant ways to base58check-encode an address.
fn base58check_encode(address: &script::ScriptAddress) -> String {
  let address = keys::Address {
    kind: address.kind,
    network: keys::Network::Mainnet,
    hash: address.hash.clone(),
  };
  let base58check = format!("{}", address);
  base58check
}

#[cfg(test)]
mod read_script_tests {
  use super::*;

  #[test]
  fn when_script_length_is_zero_then_returns_empty_script() {
    // given
    let script_bytes = [0u8];
    let mut cursor = Cursor::new(&script_bytes);

    // when
    let script = cursor.read_script().unwrap();

    // then
    assert!(script.is_empty());
  }

  #[test]
  fn when_script_length_is_non_zero_then_returns_script() {
    // given
    let script_bytes = [4u8, 0u8, 1u8, 2u8, 3u8];
    let mut cursor = Cursor::new(&script_bytes);

    // when
    let actual_script = cursor.read_script().unwrap();

    // then
    let expected_script: Box<[u8]> = Box::new([0u8, 1u8, 2u8, 3u8]);
    assert_eq!(expected_script, actual_script);
  }
}
