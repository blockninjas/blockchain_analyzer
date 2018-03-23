use std::io;
use std::io::Read;
use std::io::Cursor;
use keys;
use script;
use script::Script;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use byteorder::{LittleEndian, ReadBytesExt};
use domain::*;

const MAIN_NET_MAGIC_NUMBER: u32 = 0xD9B4BEF9;

pub fn read_block(reader: &mut Read) -> io::Result<Block> {
  let magic_number = reader.read_u32::<LittleEndian>()?;

  if magic_number != MAIN_NET_MAGIC_NUMBER {
    return Err(io::Error::new(io::ErrorKind::Other, "invalid magic number"));
  }

  let block_size = reader.read_u32::<LittleEndian>()?;

  // TODO Fix possibly truncating cast.
  let mut block_content = Box::<[u8]>::from(vec![0u8; block_size as usize]);
  reader.read_exact(&mut block_content)?;

  let mut block_content_reader = Cursor::new(block_content);

  let mut block_header = [0u8; 80];
  block_content_reader.read_exact(&mut block_header)?;

  let hash = calculate_hash(&block_header)?;

  let mut block_header_reader = Cursor::new(&block_header[..]);
  let version = block_header_reader.read_u32::<LittleEndian>()?;
  let previous_block_hash = read_hash(&mut block_header_reader)?;
  let merkle_root = read_hash(&mut block_header_reader)?;
  let creation_time = block_header_reader.read_u32::<LittleEndian>()?;
  let bits = block_header_reader.read_u32::<LittleEndian>()?;
  let nonce = block_header_reader.read_u32::<LittleEndian>()?;

  let transactions = read_transactions(&mut block_content_reader)?;

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

fn read_transactions(
  cursor: &mut Cursor<Box<[u8]>>,
) -> io::Result<Box<[Transaction]>> {
  let transaction_count = read_var_int(cursor)?;
  // TODO Fix possibly truncating cast.
  let mut transactions = Vec::with_capacity(transaction_count as usize);
  for _ in 0..transaction_count {
    let transaction = read_transaction(cursor)?;
    transactions.push(transaction);
  }
  Ok(transactions.into_boxed_slice())
}

fn read_transaction(cursor: &mut Cursor<Box<[u8]>>) -> io::Result<Transaction> {
  let start_position = cursor.position();

  let version = cursor.read_u32::<LittleEndian>()?;

  // TODO Fix possibly truncating cast.
  let input_count = read_var_int(cursor)? as u32;

  let mut inputs: Box<[Input]> = Box::new([]);
  let mut outputs: Box<[Output]> = Box::new([]);

  if input_count == 0 {
    let flags = cursor.read_u8()?;
    if flags != 0 {
      // TODO Fix possibly truncating cast.
      let input_count = read_var_int(cursor)? as u32;
      inputs = read_inputs(cursor, input_count)?;
      // TODO Fix possibly truncating cast.
      let output_count = read_var_int(cursor)? as u32;
      outputs = read_outputs(cursor, output_count)?;

      if (flags & 1u8) == 1u8 {
        for _ in 0..input_count {
          let stack_item_count = read_var_int(cursor)?;
          for _ in 0..stack_item_count {
            let stack_length = read_var_int(cursor)?;
            // TODO Fix possibly truncating cast.
            let mut stack_item =
              Box::<[u8]>::from(vec![0u8; stack_length as usize]);
            cursor.read_exact(&mut stack_item)?;
            // TODO How to interpret the stack script?
          }
        }
      }
    }
  } else {
    inputs = read_inputs(cursor, input_count)?;
    // TODO Fix possibly truncating cast.
    let output_count = read_var_int(cursor)? as u32;
    outputs = read_outputs(cursor, output_count)?;
  }

  let lock_time = cursor.read_u32::<LittleEndian>()?;

  // Calculate the length of the raw transaction data.
  let end_position = cursor.position();
  let tx_length = end_position - start_position;
  assert_ne!(tx_length, 0);

  // Get the raw transaction data.
  cursor.set_position(start_position);
  let mut tx_content = Box::<[u8]>::from(vec![0u8; tx_length as usize]);
  cursor.read_exact(&mut tx_content)?;
  assert_eq!(cursor.position(), end_position);

  // Calculate the transaction hash over the raw transaction data.
  let tx_hash = calculate_hash(&tx_content)?;

  let transaction = Transaction {
    tx_hash,
    version,
    lock_time,
    inputs,
    outputs,
    creation_time: 0,
    block_height: 0,
  };

  Ok(transaction)
}

fn read_inputs(
  reader: &mut Read,
  input_count: u32,
) -> io::Result<Box<[Input]>> {
  // TODO Fix possibly truncating cast.
  let mut inputs = Vec::with_capacity(input_count as usize);
  for _ in 0..input_count {
    let input = read_input(reader)?;
    inputs.push(input);
  }
  Ok(inputs.into_boxed_slice())
}

fn read_input(reader: &mut Read) -> io::Result<Input> {
  let previous_tx_hash = read_hash(reader)?;
  let previous_tx_output_index = reader.read_u32::<LittleEndian>()?;
  read_script(reader)?;
  let sequence_number = reader.read_u32::<LittleEndian>()?;

  let input = Input {
    sequence_number,
    previous_tx_hash,
    previous_tx_output_index,
  };

  Ok(input)
}

fn read_outputs(
  reader: &mut Read,
  output_count: u32,
) -> io::Result<Box<[Output]>> {
  // TODO Fix possibly truncating cast.
  let mut outputs = Vec::with_capacity(output_count as usize);
  for output_index in 0..output_count {
    let output = read_output(reader, output_index)?;
    outputs.push(output);
  }
  Ok(outputs.into_boxed_slice())
}

fn read_output(reader: &mut Read, index: u32) -> io::Result<Output> {
  let value = reader.read_u64::<LittleEndian>()?;
  let script = Vec::<u8>::from(read_script(reader)?);
  let addresses = read_output_addresses(script);

  let output = Output {
    index,
    value,
    addresses,
  };

  Ok(output)
}

/// Retrieves the Base58Check-encoded bitcoin addresses from an `Output`-script.
fn read_output_addresses(script: Vec<u8>) -> Box<[Address]> {
  let script = Script::from(script);
  // TODO Return a meaningful error instead of panicking.
  let addresses = script.extract_destinations().expect("Invalid addresses");
  let addresses: Vec<Address> = addresses
    .iter()
    .map(|address: &script::ScriptAddress| {
      let base58check = base58check_encode(address);
      let hash = address.hash.clone().take();

      Address { hash, base58check }
    })
    .collect();
  let addresses: Box<[Address]> = addresses.into_boxed_slice();
  addresses
}

fn calculate_hash(bytes: &[u8]) -> io::Result<Hash> {
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

fn base58check_encode(address: &script::ScriptAddress) -> String {
  // Transform the `ScriptAddress` to a `keys::Address` and leverage the
  // `Format` trait implementation of `keys::Address` to retrieve it as base58
  // encoded string.
  // TODO Investigate more elegant ways to base58-encode an address.
  let address = keys::Address {
    kind: address.kind,
    network: keys::Network::Mainnet,
    hash: address.hash.clone(),
  };
  let base58check = format!("{}", address);
  base58check
}

fn read_hash(reader: &mut Read) -> io::Result<Hash> {
  let mut hash: [u8; 32] = [0; 32];
  reader.read_exact(&mut hash)?;
  hash.reverse();
  Ok(Hash(hash))
}

fn read_var_int(reader: &mut Read) -> io::Result<u64> {
  let mut control_byte: [u8; 1] = [0];
  reader.read_exact(&mut control_byte)?;

  let var_int: u64 = if control_byte[0] < 0xFDu8 {
    control_byte[0] as u64
  } else if control_byte[0] == 0xFDu8 {
    reader.read_u16::<LittleEndian>()? as u64
  } else if control_byte[0] == 0xFEu8 {
    reader.read_u32::<LittleEndian>()? as u64
  } else {
    reader.read_u64::<LittleEndian>()?
  };

  Ok(var_int)
}

fn read_script(reader: &mut Read) -> io::Result<Box<[u8]>> {
  let script_length = read_var_int(reader)?;
  let mut script = Box::<[u8]>::from(vec![0u8; script_length as usize]);
  reader.read_exact(&mut script)?;
  Ok(script)
}

#[cfg(test)]
mod read_script_tests {
  use super::*;

  #[test]
  fn when_script_length_is_zero_then_returns_empty_script() {
    // given
    let script_bytes = [0u8];
    let mut reader = Cursor::new(&script_bytes);

    // when
    let script = read_script(&mut reader).unwrap();

    // then
    assert!(script.is_empty());
  }

  #[test]
  fn when_script_length_is_non_zero_then_returns_script() {
    // given
    let script_bytes = [4u8, 0u8, 1u8, 2u8, 3u8];
    let mut reader = Cursor::new(&script_bytes);

    // when
    let actual_script = read_script(&mut reader).unwrap();

    // then
    let expected_script: Box<[u8]> = Box::new([0u8, 1u8, 2u8, 3u8]);
    assert_eq!(expected_script, actual_script);
  }
}
