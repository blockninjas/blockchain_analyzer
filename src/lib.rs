extern crate clap;
#[macro_use] extern crate log;
extern crate crypto;

use std::fs::File;
use std::io::Cursor;
use std::io::BufReader;
use std::io::prelude::*;
use std::error::Error;
use std::result::Result;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

pub mod domain;

use domain::{Block, Transaction, Input, Output};

pub fn read_blk_files(source_path: &str) -> usize {
    let mut blk_file_counter = 0;
    loop {
        let blk_file_name = format!("blk{:05}.dat", blk_file_counter);
        let blk_file_path = format!("{}/{}", source_path, blk_file_name);
        let mut blk_file = match File::open(blk_file_path) {
            Ok(blk_file) => blk_file,
            _ => break,
        };

        info!("Read {}", blk_file_name);

        let number_of_blocks = read_blk_file(&mut blk_file);

        info!("Processed {} blocks in {}", number_of_blocks, blk_file_name);

        blk_file_counter += 1;
    }
    blk_file_counter
}

pub fn read_blk_file(blk_file: &mut File) -> usize {
    let mut buf_reader = BufReader::new(blk_file);
    let mut block_counter = 0;
    loop {
        if let Err(ref error) = read_block(&mut buf_reader) {
            if error.kind() != std::io::ErrorKind::UnexpectedEof {
                error!("Could not read file (reason: {})", error.description());
            }
            break;
        };
        block_counter += 1;
    }
    block_counter
}

pub fn read_block(reader: &mut Read) -> Result<Block, std::io::Error> {
    let magic_number = read_u32(reader)?;

    if magic_number != 0xD9B4BEF9 {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "invalid magic number"));
    }

    let block_size = read_u32(reader)?;

    // TODO Fix possibly truncating cast.
    let mut block_content = Box::<[u8]>::from(vec![0u8; block_size as usize]);
    reader.read_exact(&mut block_content)?;

    let mut block_content_reader = Cursor::new(block_content);

    let mut block_header = Box::<[u8]>::from(vec![0u8; 80]);
    block_content_reader.read_exact(&mut block_header)?;

    let hash = calculate_hash(&block_header)?;

    let mut block_header_reader = Cursor::new(block_header);
    let version = read_u32(&mut block_header_reader)?;
    let previous_block_hash = read_hash(&mut block_header_reader)?;
    let merkle_root = read_hash(&mut block_header_reader)?;
    let creation_time = read_u32(&mut block_header_reader)?;
    let bits = read_u32(&mut block_header_reader)?;
    let nonce = read_u32(&mut block_header_reader)?;

    let transactions = read_transactions(&mut block_content_reader)?;

    let block = Block {
        block_height: 0,
        creation_time,
        hash,
        merkle_root,
        bits,
        nonce,
        previous_block_hash,
        version,
        transactions,
    };

    debug!("Parsed block: {:#?}", block);

    Ok(block)
}

pub fn calculate_hash(bytes: &[u8]) -> Result<String, std::io::Error>  {
    let mut sha = Sha256::new();

    // first hash round
    sha.input(&bytes);
    let mut first_hash = Box::<[u8]>::from(vec![0u8; 32]);
    sha.result(&mut first_hash);

    // second hash round
    sha.reset();
    sha.input(&first_hash);
    let mut second_hash = Box::<[u8]>::from(vec![0u8; 32]);
    sha.result(&mut second_hash);

    let hash = to_big_endian_hex(&second_hash);
    Ok(hash)
}

pub fn read_transactions(cursor: &mut Cursor<Box<[u8]>>) -> Result<Box<[Transaction]>, std::io::Error> {
    let transaction_count = read_var_int(cursor)?;
    // TODO Fix possibly truncating cast.
    let mut transactions = Vec::with_capacity(transaction_count as usize);
    for _ in 0..transaction_count {
        let transaction = read_transaction(cursor)?;
        transactions.push(transaction);
    }
    Ok(transactions.into_boxed_slice())
}

pub fn read_transaction(cursor: &mut Cursor<Box<[u8]>>) -> Result<Transaction, std::io::Error> {
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
                        let mut stack_item = Box::<[u8]>::from(vec![0u8; stack_length as usize]);
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
        block_height: 0,
        version,
        creation_time: 0,
        lock_time,
        inputs,
        outputs,
    };

    Ok(transaction)
}

pub fn read_inputs(reader: &mut Read, input_count: u32) -> Result<Box<[Input]>, std::io::Error> {
    // TODO Fix possibly truncating cast.
    let mut inputs = Vec::with_capacity(input_count as usize);
    for _ in 0..input_count {
        let input = read_input(reader)?;
        inputs.push(input);
    }
    Ok(inputs.into_boxed_slice())
}

pub fn read_input(reader: &mut Read) -> Result<Input, std::io::Error> {
    let previous_tx_hash = read_hash(reader)?;
    let previous_tx_output_index = read_u32(reader)?;
    let script = read_script(reader)?;
    let sequence_number = read_u32(reader)?;

    let input = Input {
        sequence_number,
        script,
        previous_tx_hash,
        previous_tx_output_index,
    };

    Ok(input)
}

pub fn read_outputs(reader: &mut Read, output_count: u32) -> Result<Box<[Output]>, std::io::Error> {
    // TODO Fix possibly truncating cast.
    let mut outputs = Vec::with_capacity(output_count as usize);
    for output_index in 0..output_count {
        let output = read_output(reader, output_index)?;
        outputs.push(output);
    }
    Ok(outputs.into_boxed_slice())
}

pub fn read_output(reader: &mut Read, index: u32) -> Result<Output, std::io::Error> {
    let value = read_u64(reader)?;
    let script = read_script(reader)?;

    let output = Output {
        index,
        script,
        value,
    };

    Ok(output)
}

pub fn read_hash(reader: &mut Read) -> Result<String, std::io::Error> {
    let mut hash: [u8; 32] = [0; 32];
    reader.read_exact(&mut hash)?;
    Ok(to_big_endian_hex(&hash))
}

pub fn to_big_endian_hex(little_endian_bytes: &[u8]) -> String {
    little_endian_bytes.iter()
        .rev()
        .map(|b| format!("{:02X}", b))
        .collect()
}

#[cfg(test)]
mod to_big_endian_hex_tests {
    use super::to_big_endian_hex;

    #[test]
    fn returns_big_endian_hex() {
        // given
        let little_endian_bytes = [0x89, 0xAB, 0xCD, 0xEF];

        // when
        let actual_hex = to_big_endian_hex(&little_endian_bytes);

        // then
        let expected_hex = "EFCDAB89";
        assert_eq!(expected_hex, actual_hex);
    }

    #[test]
    fn does_not_truncate_leading_zeros() {
        // given
        let little_endian_bytes = [0x01, 0x00];

        // when
        let actual_hex = to_big_endian_hex(&little_endian_bytes);

        // then
        let expected_hex = "0001";
        assert_eq!(expected_hex, actual_hex);
    }
}

pub fn read_u8(reader: &mut Read) -> Result<u8, std::io::Error> {
    let mut number: [u8; 1] = [0];
    reader.read_exact(&mut number)?;
    Ok(number[0])
}

pub fn read_u16(reader: &mut Read) -> Result<u16, std::io::Error> {
    let mut number: [u8; 2] = [0; 2];
    reader.read_exact(&mut number)?;
    Ok(to_big_endian::<u16>(&number))
}

pub fn read_u32(reader: &mut Read) -> Result<u32, std::io::Error> {
    let mut number: [u8; 4] = [0; 4];
    reader.read_exact(&mut number)?;
    Ok(to_big_endian::<u32>(&number))
}

pub fn read_u64(reader: &mut Read) -> Result<u64, std::io::Error> {
    let mut number: [u8; 8] = [0; 8];
    reader.read_exact(&mut number)?;
    Ok(to_big_endian::<u64>(&number))
}

pub fn read_var_int(reader: &mut Read) -> Result<u64, std::io::Error> {
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

pub fn read_script(reader: &mut Read) -> Result<Box<[u8]>, std::io::Error> {
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

pub fn to_big_endian<T>(little_endian_bytes: &[u8]) -> T
    where T: From<u8> + std::ops::Shl<u8> +
             From<<T as std::ops::Shl<u8>>::Output> + std::ops::Add +
             From<<T as std::ops::Add>::Output>
{
    little_endian_bytes.iter()
        .rev()
        .fold(T::from(0u8), |acc, &x| T::from(T::from(T::shl(acc, 8u8)) + T::from(x)))
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
