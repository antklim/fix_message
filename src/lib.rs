use std::error::Error;
use std::fmt;

use self::FIXMessageParseError::{ProtocolVersionNotFound, UnsupportedProtocolVersion, InvalidChecksum};

#[derive(PartialEq, Debug)]
pub enum FIXMessageParseError {
  ProtocolVersionNotFound,
  UnsupportedProtocolVersion(String),
  InvalidChecksum,
}

impl fmt::Display for FIXMessageParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      UnsupportedProtocolVersion(ref version) => write!(f, "{}: {}", self.description(), version),
      _ => write!(f, "{}", self.description()),
    }
  }
}

impl Error for FIXMessageParseError {
  fn description(&self) -> &str {
    match *self {
      ProtocolVersionNotFound => "FIX message protocol version not found",
      UnsupportedProtocolVersion(..) => "Unsupported FIX protocol version",
      InvalidChecksum => "Invalid FIX message checksum.",
    }
  }
}

pub struct FIXMessage {
  version: String,
  header: FIXMessageHeader,
  body: FIXMessageBody,
  footer: FIXMessageFooter,
}

pub struct FIXMessageHeader {
  version: String,
  body: String,
}

pub struct FIXMessageBody {
  version: String,
  body: String,
}

pub struct FIXMessageFooter {
  version: String,
  body: String,
}

fn get_protocol_version(inbound_message: &str) -> Result<String, FIXMessageParseError> {
  Ok(String::from("test"))
}

pub fn parse(inbound_message: &str) -> Result<FIXMessage, FIXMessageParseError> {
  let fix_version = "4.2";
  let fix_message_header = FIXMessageHeader {version: String::from(fix_version), body: "fix_header".to_string()};
  let fix_message_body = FIXMessageBody {version: String::from(fix_version), body: "fix_body".to_string()};
  let fix_message_footer = FIXMessageFooter {version: String::from(fix_version), body: "fix_footer".to_string()};
  let fix_message = FIXMessage {
    version: String::from(fix_version),
    header: fix_message_header,
    body: fix_message_body,
    footer: fix_message_footer
  };

  Ok(fix_message)
}

pub fn generate(outbound_messge: FIXMessage) -> String {
  return "hello_world".to_string();
}

#[cfg(test)]
mod tests {
  use super::{parse, generate};

  #[test]
  fn it_should_parse_fix_message() {
    assert_eq!(true, true);
  }

  #[test]
  fn it_should_generate_fix_message() {
    assert_eq!(true, true);
  }
}
