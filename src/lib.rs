//! The `fix_message` crate provides functions that parse, validate and generate
//! FIX messages. These functions are agnostic to FIX protocol version.
//!
//! Here validated only those message fields which common for all versions of
//! FIX protocol.
//! This crate should be used as a middleware for parsing and initial validation
//! of messages. And as middleware for the final validation, checksum
//! calculation and FIX message generation.
//!
//! Message body or additional fields must be validated in version specific
//! crates.

extern crate fix_checksum;

use std::result;

pub use self::errors::*;
// pub use self::generator::*;
pub use self::parser::*;
pub use self::structs::*;
pub use self::FIXMessageError::*;

pub const FIX_MESSAGE_DELIMITER: char = '\x01';
pub const FIX_MESSAGE_FIELD_DELIMITER: char = '\x3D';

pub type FIXMessageResult<T> = result::Result<T, FIXMessageError>;

mod errors;
// mod generator;
mod parser;
mod structs;
