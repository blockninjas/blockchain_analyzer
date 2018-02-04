extern crate clap;
#[macro_use] extern crate log;

use std::fs::File;
use std::io::Cursor;
use std::io::BufReader;
use std::io::prelude::*;
use std::error::Error;
use std::result::Result;

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
    let mut block_content = vec![0u8; block_size as usize].into_boxed_slice();
    reader.read_exact(&mut block_content)?;

    let mut block_content_reader = Cursor::new(&block_content);

    let version = read_u32(&mut block_content_reader)?;
    let previous_block_hash = read_hash(&mut block_content_reader)?;
    let merkle_root = read_hash(&mut block_content_reader)?;
    let creation_time = read_u32(&mut block_content_reader)?;
    let bits = read_u32(&mut block_content_reader)?;
    let nonce = read_u32(&mut block_content_reader)?;
    let transactions = read_transactions(&mut block_content_reader)?;

    let block = Block {
        block_height: 0,
        creation_time,
        hash: String::new(),
        merkle_root,
        bits,
        nonce,
        previous_block_hash,
        version,
        transactions,
    };

    Ok(block)
}

pub fn read_transactions(reader: &mut Read) -> Result<Box<[Transaction]>, std::io::Error> {
    let transaction_count = read_var_int(reader)?;
    // TODO Fix possibly truncating cast.
    let mut transactions = Vec::with_capacity(transaction_count as usize);
    for _ in 0..transaction_count {
        let transaction = read_transaction(reader)?;
        transactions.push(transaction);
    }
    Ok(transactions.into_boxed_slice())
}

pub fn read_transaction(reader: &mut Read) -> Result<Transaction, std::io::Error> {
    let version = read_u32(reader)?;

    // TODO Fix possibly truncating cast.
    let input_count = read_var_int(reader)? as u32;

    let mut inputs: Box<[Input]> = Box::new([]);
    let mut outputs: Box<[Output]> = Box::new([]);

    if input_count == 0 {
        let flags = read_u8(reader)?;
        if flags != 0 {
            // TODO Fix possibly truncating cast.
            let input_count = read_var_int(reader)? as u32;
            inputs = read_inputs(reader, input_count)?;
            // TODO Fix possibly truncating cast.
            let output_count = read_var_int(reader)? as u32;
            outputs = read_outputs(reader, output_count)?;

            if (flags & 1u8) == 1u8 {
                for _ in 0..input_count {
                    let stack_item_count = read_var_int(reader)?;
                    for _ in 0..stack_item_count {
                        let stack_length = read_var_int(reader)?;
                        // TODO Fix possibly truncating cast.
                        let mut stack_item = vec![0u8; stack_length as usize].into_boxed_slice();
                        reader.read_exact(&mut stack_item)?;
                        // TODO How to interpret the stack script?
                    }
                }
            }
        }
    } else {
        inputs = read_inputs(reader, input_count)?;
        // TODO Fix possibly truncating cast.
        let output_count = read_var_int(reader)? as u32;
        outputs = read_outputs(reader, output_count)?;
    }

    let lock_time = read_u32(reader)?;

    let transaction = Transaction {
        tx_hash: String::new(),
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
        .map(|b| format!("{:X}", b))
        .collect()
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

pub fn to_big_endian<T>(little_endian_bytes: &[u8]) -> T
    where T: From<u8> + std::ops::Shl<u8> +
             From<<T as std::ops::Shl<u8>>::Output> + std::ops::Add +
             From<<T as std::ops::Add>::Output>
{
    little_endian_bytes.iter()
        .rev()
        .fold(T::from(0u8), |acc, &x| T::from(T::from(T::shl(acc, 8u8)) + T::from(x)))
}

pub fn read_script(reader: &mut Read) -> Result<Box<[u8]>, std::io::Error> {
    let script_length = read_var_int(reader)?;
    let mut script = vec![0u8; script_length as usize].into_boxed_slice();
    reader.read_exact(&mut script)?;
    Ok(script)
}
