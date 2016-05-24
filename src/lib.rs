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
use fix_checksum::validate as validate_checksum;
use fix_checksum::FIXChecksumValidatorError;

pub use self::errors::*;
pub use self::structs::*;
pub use self::FIXMessageError::*;

pub const FIX_MESSAGE_DELIMITER: char = '\x01';
pub const FIX_MESSAGE_FIELD_DELIMITER: char = '\x3D';

fn brew_message(message_parts: Vec<&str>, delimiter: &str) -> String {
  return message_parts
    .iter()
    .fold(String::new(), |message, message_part| message.to_string() + message_part + delimiter);
}

/// This function validates and parses FIX message
///
/// # Examples
pub fn parse(inbound_message: &str) -> FIXMessageResult<FIXMessage> {

  validate_checksum(inbound_message)
    .map_err(|err: FIXChecksumValidatorError| InvalidChecksum(err))
    .and_then(|is_valid_value: bool| if is_valid_value { Ok(()) } else { Err(InvalidChecksumValue) })
    .and(
      Ok(FIXMessage {
        version: "FIX".to_string(),
        data: vec![]
      })
    )
}

/// This function validates and generates FIX message
///
/// # Examples
pub fn generate(outbound_messge: FIXMessage) -> FIXMessageResult<String> {
  unimplemented!();
}

pub type FIXMessageResult<T> = result::Result<T, FIXMessageError>;

mod errors;
mod structs;

#[cfg(test)]
mod tests {
  extern crate fix_checksum;

  use super::{FIX_MESSAGE_DELIMITER, parse, generate, brew_message};
  use super::FIXMessageError::*;
  use fix_checksum::FIXChecksumValidatorError::*;

  #[test]
  fn it_should_complain_when_checksum_not_found() {
    let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
      "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28"];
    let message: String = brew_message(message_parts, &(FIX_MESSAGE_DELIMITER.to_string()));

    assert_eq!(parse(&message).unwrap_err(), InvalidChecksum(ChecksumFieldNotFound));
  }

  #[test]
  fn it_should_complain_when_checksum_format_is_not_valid() {
    let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
      "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=2ZZ"];
    let message: String = brew_message(message_parts, &(FIX_MESSAGE_DELIMITER.to_string()));

    assert_eq!(parse(&message).unwrap_err(),
      InvalidChecksum(ChecksumFieldInvalidFormat("2ZZ".parse::<u32>().unwrap_err())));
  }

  #[test]
  fn it_should_complain_when_checksum_is_incorrect() {
    let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
      "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=231"];
    let message: String = brew_message(message_parts, &(FIX_MESSAGE_DELIMITER.to_string()));

    assert_eq!(parse(&message).unwrap_err(), InvalidChecksumValue);
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
