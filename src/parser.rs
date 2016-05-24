use super::{FIXMessageResult, FIXMessage, FIX_MESSAGE_DELIMITER};
use super::FIXMessageError::*;
use fix_checksum::validate as validate_checksum;
use fix_checksum::FIXChecksumValidatorError;

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
        data: vec![]//inbound_message.split(FIX_MESSAGE_DELIMITER).collect()
      })
    )
}

#[cfg(test)]
mod tests {
  use super::super::{FIX_MESSAGE_DELIMITER};
  use super::super::FIXMessageError::*;
  use super::*;
  use fix_checksum::FIXChecksumValidatorError::*;

  #[test]
  fn it_should_complain_when_checksum_not_found() {
    let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
      "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28"];
    let message: String = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));

    assert_eq!(parse(&message).unwrap_err(), InvalidChecksum(ChecksumFieldNotFound));
  }

  #[test]
  fn it_should_complain_when_checksum_format_is_not_valid() {
    let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
      "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=2ZZ"];
    let message: String = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));

    assert_eq!(parse(&message).unwrap_err(),
      InvalidChecksum(ChecksumFieldInvalidFormat("2ZZ".parse::<u32>().unwrap_err())));
  }

  #[test]
  fn it_should_complain_when_checksum_is_incorrect() {
    let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
      "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=231"];
    let message: String = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));

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
}
