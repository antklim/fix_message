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

use std::error::Error;
use std::fmt;
use std::result;

use self::FIXMessageError::{ProtocolVersionNotFound, InvalidChecksum};

const FIX_MESSAGE_DELIMITER: char = '\x01';
const FIX_MESSAGE_FIELD_DELIMITER: char = '\x3D';

#[derive(PartialEq, Debug)]
pub enum FIXMessageError {
  ProtocolVersionNotFound,
  InvalidChecksum,
}

impl fmt::Display for FIXMessageError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      _ => write!(f, "{}", self.description()),
    }
  }
}

impl Error for FIXMessageError {
  fn description(&self) -> &str {
    match *self {
      ProtocolVersionNotFound => "FIX message protocol version not found",
      InvalidChecksum => "Invalid FIX message checksum.",
    }
  }
}

/// This structure represents field/value pair of FIX message
pub struct FIXMessageField {
  field: String,
  value: String
}

/// This structure represents the whole FIX message
///
/// ### Parsing FIX message
/// When parsing FIX messaged then field/value pairs stored in order they
/// were in the message.
///
/// ### Generating FIX messages
/// When generating FIX message then field/value pairs will be concatenated
/// in the order they stored in vector.
/// First two fileds (8, 9) should not be provided in data (will be ignored if
/// found). Field `8` - `BeginString` will be automatically added to the message
/// header and have `FIXMessage.version` value. Field `9` - `BodyLength` will be
/// automatically calculated and added to the message header.
pub struct FIXMessage {
  version: String,
  data: Vec<FIXMessageField>
}

/// This function validates and parses FIX message
///
/// # Examples
pub fn parse(inbound_message: &str) -> FIXMessageResult<FIXMessage> {
  unimplemented!();
}

/// This function validates and generates FIX message
///
/// # Examples
pub fn generate(outbound_messge: FIXMessage) -> FIXMessageResult<String> {
  unimplemented!();
}

pub type FIXMessageResult<T> = result::Result<T, FIXMessageError>;

#[cfg(test)]
mod tests {
  use super::{parse, generate};

  #[test]
  fn it_should_complain_when_checksum_not_found() {
    // checksum must always be the last
    unimplemented!();
  }

  #[test]
  fn it_should_complain_when_checksum_format_is_not_valid() {
    unimplemented!();
  }

  #[test]
  fn it_should_complain_when_checksum_is_incorrect() {
    unimplemented!();
  }

  #[test]
  fn it_should_complain_when_not_all_required_header_fields_presented() {
    // should split message by delimiter
    unimplemented!();
  }

  #[test]
  fn it_should_complain_when_first_three_fields_in_wrong_order() {
    // should split message by delimiter
    unimplemented!();
  }

  #[test]
  fn it_should_parse_fix_message() {
    // should split message by delimiter
    unimplemented!();
  }

  #[test]
  fn it_should_generate_fix_message() {
    // concatenate message
    unimplemented!();
  }
}
