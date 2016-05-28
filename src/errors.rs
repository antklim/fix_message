use std::error::Error;
use std::fmt;
use fix_checksum::FIXChecksumValidatorError;
use self::FIXMessageError::*;

#[derive(PartialEq, Debug)]
pub enum FIXMessageError <'a> {
  InvalidChecksum(FIXChecksumValidatorError),
  InvalidChecksumValue,
  InvalidFieldStructure,
  InvalidFirstField(&'a str),
  InvalidSecondField(&'a str),
  InvalidThirdField(&'a str),
  NotAllRequiredFieldsFound,
}

impl <'a> fmt::Display for FIXMessageError<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      InvalidChecksum(ref err) => write!(f, "{}: {}", self.description(), err),
      InvalidFirstField(ref tag) |
      InvalidSecondField(ref tag) |
      InvalidThirdField(ref tag) => write!(f, "{}: {}", self.description(), tag),
      _ => write!(f, "{}", self.description()),
    }
  }
}

impl <'a> Error for FIXMessageError<'a> {
  fn description(&self) -> &str {
    match *self {
      InvalidChecksum(..) => "Invalid FIX message checksum",
      InvalidChecksumValue => "Invalid value of FIX message checksum",
      InvalidFieldStructure => "Invalid structure of FIX message field, should be <tag>=<value>",
      InvalidFirstField(..) => "Invalid first field, should be `8` but found",
      InvalidSecondField(..) => "Invalid first field, should be `9` but found",
      InvalidThirdField(..) => "Invalid first field, should be `35` but found",
      NotAllRequiredFieldsFound => "Not all required fields found",
    }
  }
}
