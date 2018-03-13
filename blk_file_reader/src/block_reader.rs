use std::fs::File;
use std::io;
use std::ops;
use std::io::Read;
use std::io::Cursor;
use std::io::BufReader;
use keys;
use script;
use script::Script;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use domain::*;
use BlockRead;

const MAIN_NET_MAGIC_NUMBER: u32 = 0xD9B4BEF9;

/// Allows for reading `Block`s from a .blk file.
pub struct BlockReader {
  reader: Box<Read>,
}

impl<'a> BlockReader {
  pub fn new(reader: Box<Read>) -> BlockReader {
    BlockReader { reader }
  }

  pub fn from_blk_file(blk_file_path: &str) -> BlockReader {
    // TODO Return error instead of panicking.
    let blk_file = File::open(blk_file_path).unwrap();
    let reader = Box::new(BufReader::new(blk_file));
    BlockReader { reader }
  }
}

impl BlockRead for BlockReader {
  fn skip(&mut self, number_of_blocks_to_skip: usize) -> io::Result<()> {
    // TODO Avoid unnecessary parsing of full block contents.
    for _ in 0..number_of_blocks_to_skip {
      let _ = self.read()?;
    }
    Ok(())
  }

  // TODO Introduce `has_next()` to be able to determine if a further call to
  //      `read()` is sane and avoid returning `UnexpecedEof`.
  fn read(&mut self) -> io::Result<Block> {
    read_block(&mut self.reader)
  }
}

fn read_block(reader: &mut Read) -> io::Result<Block> {
  let magic_number = read_u32(reader)?;

  if magic_number != MAIN_NET_MAGIC_NUMBER {
    return Err(io::Error::new(io::ErrorKind::Other, "invalid magic number"));
  }

  let block_size = read_u32(reader)?;

  // TODO Fix possibly truncating cast.
  let mut block_content = Box::<[u8]>::from(vec![0u8; block_size as usize]);
  reader.read_exact(&mut block_content)?;

  let mut block_content_reader = Cursor::new(block_content);

  let mut block_header = [0u8; 80];
  block_content_reader.read_exact(&mut block_header)?;

  let hash = calculate_hash(&block_header)?;

  let mut block_header_reader = Cursor::new(&block_header[..]);
  let version = read_u32(&mut block_header_reader)?;
  let previous_block_hash = read_hash(&mut block_header_reader)?;
  let merkle_root = read_hash(&mut block_header_reader)?;
  let creation_time = read_u32(&mut block_header_reader)?;
  let bits = read_u32(&mut block_header_reader)?;
  let nonce = read_u32(&mut block_header_reader)?;

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

  let version = read_u32(cursor)?;

  // TODO Fix possibly truncating cast.
  let input_count = read_var_int(cursor)? as u32;

  let mut inputs: Box<[Input]> = Box::new([]);
  let mut outputs: Box<[Output]> = Box::new([]);

  if input_count == 0 {
    let flags = read_u8(cursor)?;
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

  let lock_time = read_u32(cursor)?;

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
  let previous_tx_output_index = read_u32(reader)?;
  read_script(reader)?;
  let sequence_number = read_u32(reader)?;

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
  let value = read_u64(reader)?;
  let script = Vec::<u8>::from(read_script(reader)?);
  let addresses = read_output_addresses(script);

  let output = Output {
    index,
    value,
    addresses,
  };

  Ok(output)
}

/// Retrieves the base58-encoded bitcoin addresses from an `Output`-script.
fn read_output_addresses(script: Vec<u8>) -> Box<[Address]> {
  let script = Script::from(script);
  // TODO Return a meaningful error instead of panicking.
  let addresses = script.extract_destinations().expect("Invalid addresses");
  let addresses: Vec<Address> = addresses
    .iter()
    .map(|address: &script::ScriptAddress| {
      let base58_string = base58_encode(address);
      let hash = address.hash.clone().take();

      Address {
        hash,
        base58_string,
      }
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

fn base58_encode(address: &script::ScriptAddress) -> String {
  // Transform the `ScriptAddress` to a `keys::Address` and leverage the
  // `Format` trait implementation of `keys::Address` to retrieve it as base58
  // encoded string.
  // TODO Investigate more elegant ways to base58-encode an address.
  let address = keys::Address {
    kind: address.kind,
    network: keys::Network::Mainnet,
    hash: address.hash.clone(),
  };
  let base58_string = format!("{}", address);
  base58_string
}

fn read_hash(reader: &mut Read) -> io::Result<Hash> {
  let mut hash: [u8; 32] = [0; 32];
  reader.read_exact(&mut hash)?;
  hash.reverse();
  Ok(Hash(hash))
}

fn read_u8(reader: &mut Read) -> io::Result<u8> {
  let mut number: [u8; 1] = [0];
  reader.read_exact(&mut number)?;
  Ok(number[0])
}

fn read_u16(reader: &mut Read) -> io::Result<u16> {
  let mut number: [u8; 2] = [0; 2];
  reader.read_exact(&mut number)?;
  Ok(to_big_endian::<u16>(&number))
}

fn read_u32(reader: &mut Read) -> io::Result<u32> {
  let mut number: [u8; 4] = [0; 4];
  reader.read_exact(&mut number)?;
  Ok(to_big_endian::<u32>(&number))
}

fn read_u64(reader: &mut Read) -> io::Result<u64> {
  let mut number: [u8; 8] = [0; 8];
  reader.read_exact(&mut number)?;
  Ok(to_big_endian::<u64>(&number))
}

fn read_var_int(reader: &mut Read) -> io::Result<u64> {
  let mut control_byte: [u8; 1] = [0];
  reader.read_exact(&mut control_byte)?;

  let var_int: u64 = if control_byte[0] < 0xFDu8 {
    control_byte[0] as u64
  } else if control_byte[0] == 0xFDu8 {
    read_u16(reader)? as u64
  } else if control_byte[0] == 0xFEu8 {
    read_u32(reader)? as u64
  } else {
    read_u64(reader)?
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

fn to_big_endian<T>(little_endian_bytes: &[u8]) -> T
where
  T: From<u8>
    + ops::Shl<u8>
    + From<<T as ops::Shl<u8>>::Output>
    + ops::Add
    + From<<T as ops::Add>::Output>,
{
  little_endian_bytes
    .iter()
    .rev()
    .fold(T::from(0u8), |acc, &x| {
      T::from(T::from(T::shl(acc, 8u8)) + T::from(x))
    })
}

#[cfg(test)]
mod to_big_endian_u32_tests {
  use super::to_big_endian;

  #[test]
  fn when_passed_zeroes_then_returns_zero() {
    // given
    let zeroes = [0u8; 4];

    // when
    let actual = to_big_endian::<u32>(&zeroes);

    // then
    assert_eq!(0u32, actual);
  }

  #[test]
  fn when_passed_one_then_returns_one() {
    // given
    let zeroes = [1u8, 0u8, 0u8, 0u8];

    // when
    let actual = to_big_endian::<u32>(&zeroes);

    // then
    assert_eq!(1u32, actual);
  }

  #[test]
  fn can_parse_magic_number() {
    // given
    let little_endian_magic_number = [0xF9u8, 0xBEu8, 0xB4u8, 0xD9u8];

    // when
    let actual = to_big_endian::<u32>(&little_endian_magic_number);

    // then
    let big_endian_magic_number = 0xD9B4BEF9u32;
    assert_eq!(big_endian_magic_number, actual);
  }
}

#[cfg(test)]
mod to_big_endian_u64_tests {
  use super::to_big_endian;

  #[test]
  fn when_passed_zeroes_then_returns_zero() {
    // given
    let zeroes = [0u8; 8];

    // when
    let actual = to_big_endian::<u64>(&zeroes);

    // then
    assert_eq!(0u64, actual);
  }

  #[test]
  fn when_passed_one_then_returns_one() {
    // given
    let zeroes = [1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8];

    // when
    let actual = to_big_endian::<u64>(&zeroes);

    // then
    assert_eq!(1u64, actual);
  }
}
