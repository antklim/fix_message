//! The `fix_message` crate provides functions that parse, validate and generate
//! FIX messages. These functions are agnostic to FIX protocol version.
//!
//! The crate provides methods to validate only those message fields which common
//! for all versions of FIX protocol.
//! This crate should be used as a middleware for parsing and initial validation
//! of messages. And as middleware for the final validation, checksum
//! calculation and FIX message generation.
//!
//! Message body or additional fields must be validated in version specific
//! crates.
//!
//! # Examples
//! ## Parse message
//! ```
//! use fix_message::*;
//!
//! let message_parts = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
//!   "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=236"];
//! let message = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));
//!
//! let expected_fix_message = FIXMessage {
//!   version: "FIX.4.2".to_string(),
//!   data: vec![
//!     FIXMessageField { tag: "8".to_string(), value: "FIX.4.2".to_string() } ,
//!     FIXMessageField { tag: "9".to_string(), value: "73".to_string() } ,
//!     FIXMessageField { tag: "35".to_string(), value: "0".to_string() } ,
//!     FIXMessageField { tag: "49".to_string(), value: "BRKR".to_string() } ,
//!     FIXMessageField { tag: "56".to_string(), value: "INVMGR".to_string() } ,
//!     FIXMessageField { tag: "34".to_string(), value: "235".to_string() } ,
//!     FIXMessageField { tag: "52".to_string(), value: "19980604-07:58:28".to_string() } ,
//!     FIXMessageField { tag: "112".to_string(), value: "19980604-07:58:28".to_string() } ,
//!     FIXMessageField { tag: "10".to_string(), value: "236".to_string() }
//!   ]
//! };
//!
//!  assert_eq!(parse(&message).unwrap(), expected_fix_message);
//! ```
//! Please refer to `parse` function documentation for more examples
extern crate fix_checksum;

use std::result;

pub use self::constants::*;
pub use self::errors::*;
// pub use self::generator::*;
pub use self::parser::*;
pub use self::structs::*;
// pub use self::traits::*;
pub use self::FIXMessageError::*;

pub type FIXMessageResult<'a, T> = result::Result<T, FIXMessageError<'a>>;

mod constants;
mod errors;
pub mod fix_message_fields;

// mod generator;
mod parser;
mod structs;
// mod traits;
