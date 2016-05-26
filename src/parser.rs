use super::{FIXMessageResult, FIXMessage, FIXMessageField};
use super::{FIX_MESSAGE_DELIMITER, FIX_MESSAGE_FIELD_DELIMITER};
use super::FIXMessageError::*;
use fix_checksum::validate as validate_checksum;
use fix_checksum::FIXChecksumValidatorError;

// 8 , 9, 35, 49, 56, 34, 52
fn validate_message_tags(message_fields: Vec<&str>) -> FIXMessageResult<Vec<FIXMessageField>> {
  let mut fix_message_fields: Vec<FIXMessageField> = Vec::with_capacity(message_fields.len());

  for (index, field) in message_fields.iter().enumerate() {
    let field_parts = field.split(FIX_MESSAGE_FIELD_DELIMITER).collect::<Vec<&str>>();

    // TODO: should be the other error!!!
    if field_parts.len() != 2 { return Err(ProtocolVersionNotFound); }

    let (tag, value) = (field_parts[0], field_parts[1]);

    match index {
      0 => (),
      1 => (),
      2 => (),
      _ => (),
    }

    fix_message_fields.push(FIXMessageField {tag: tag.to_string(), value: value.to_string()});
  }

  Ok(fix_message_fields)
}

/// This function validates and parses FIX message
///
/// # Examples
pub fn parse(inbound_message: &str) -> FIXMessageResult<FIXMessage> {
  validate_checksum(inbound_message)
    .map_err(|err: FIXChecksumValidatorError| InvalidChecksum(err))
    .and_then(|is_valid_value: bool| {
      if is_valid_value {
        Ok(inbound_message.split(FIX_MESSAGE_DELIMITER).collect::<Vec<&str>>())
      } else {
        Err(InvalidChecksumValue)
      }
    })
    .and_then(validate_message_tags)
    .and_then(|fix_message_fields: Vec<FIXMessageField>| Ok(FIXMessage {
      version: "FIX".to_string(),
      data: fix_message_fields
    }))
}


#[cfg(test)]
mod tests {
  use super::super::{FIXMessage, FIX_MESSAGE_DELIMITER};
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
    let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
      "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=236"];
    let message: String = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));

    let fix_message = FIXMessage {
      version: "FIX".to_string(),
      data: vec![]
    };

    assert_eq!(parse(&message).unwrap(), fix_message);
  }
}
