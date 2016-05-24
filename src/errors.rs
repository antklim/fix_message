use std::error::Error;
use std::fmt;
use fix_checksum::FIXChecksumValidatorError;
use self::FIXMessageError::*;

#[derive(PartialEq, Debug)]
pub enum FIXMessageError {
  ProtocolVersionNotFound,
  InvalidChecksum(FIXChecksumValidatorError),
  InvalidChecksumValue,
}

impl fmt::Display for FIXMessageError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      InvalidChecksum(ref err) => write!(f, "{}: {}", self.description(), err),
      _ => write!(f, "{}", self.description()),
    }
  }
}

impl Error for FIXMessageError {
  fn description(&self) -> &str {
    match *self {
      ProtocolVersionNotFound => "FIX message protocol version not found.",
      InvalidChecksum(..) => "Invalid FIX message checksum",
      InvalidChecksumValue => "Invalid value of FIX message checksum",
    }
  }
}
