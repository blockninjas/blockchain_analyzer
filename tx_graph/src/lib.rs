//! # Tx Graph
//!
//! The aim of the transaction graph (or "tx graph" for short) is to provide
//! convenient data-structures and an efficient data-store that are amenable for
//! large-scale analyses of the bitcoin blockchain.

extern crate memmap;
extern crate redis;

pub mod domain;
pub mod repository;
