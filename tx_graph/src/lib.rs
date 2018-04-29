//! # Tx Graph
//!
//! The aim of the transaction graph (or "tx graph" for short) is to provide
//! convenient data-structures and an efficient data-store that are amenable for
//! large-scale analyses of the bitcoin blockchain.

extern crate byteorder;
extern crate memmap;
extern crate redis;

pub mod read;
pub mod write;
pub mod memory_mapping;
