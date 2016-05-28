use super::{
  FIXMessageResult, FIXMessage, FIXMessageField,
  FIX_MESSAGE_DELIMITER, FIX_MESSAGE_FIELD_DELIMITER,
  FIX_BEGIN_STRING, FIX_BODY_LENGTH, FIX_MSG_TYPE,
  FIX_SENDER_COMP_ID, FIX_TARGET_COMP_ID, FIX_MSG_SEQ_NUM, FIX_SENDING_TIME
};
use super::FIXMessageError::*;

use fix_checksum::validate as validate_checksum;
use fix_checksum::FIXChecksumValidatorError;

fn validate_message_tags(message_fields: Vec<&str>) -> FIXMessageResult<Vec<FIXMessageField>> {
  let mut fix_message_fields: Vec<FIXMessageField> = Vec::with_capacity(message_fields.len());

  let mut is_required_field_found = vec![false; 7];

  for (index, field) in message_fields.iter().enumerate() {
    let field_parts = field.splitn(2, FIX_MESSAGE_FIELD_DELIMITER).collect::<Vec<&str>>();
    let (tag, value) = (field_parts[0], field_parts[1]);
    if tag == "" || value == "" { return Err(InvalidFieldStructure); }

    let mut required_field_found_index: Option<usize> = None;

    match index {
      0 => {
        if tag != FIX_BEGIN_STRING {
          return Err(InvalidFirstField(tag))
        }
        required_field_found_index = Some(0)
      },
      1 => {
        if tag != FIX_BODY_LENGTH {
          return Err(InvalidSecondField(tag))
        }
        required_field_found_index = Some(1)
      },
      2 => {
        if tag != FIX_MSG_TYPE {
          return Err(InvalidThirdField(tag))
        }
        required_field_found_index = Some(2)
      },
      _ =>
        match tag {
          FIX_SENDER_COMP_ID => required_field_found_index = Some(3),
          FIX_TARGET_COMP_ID => required_field_found_index = Some(4),
          FIX_MSG_SEQ_NUM => required_field_found_index = Some(5),
          FIX_SENDING_TIME => required_field_found_index = Some(6),
          _ => (),
        },
    }

    if required_field_found_index.is_some() {
      is_required_field_found[ required_field_found_index.unwrap() ] = true;
    }
    fix_message_fields.push(FIXMessageField {tag: tag.to_string(), value: value.to_string()});
  }

  let is_all_required_fileds_found = is_required_field_found
    .iter()
    .fold(true, |result, is_field_found| result && *is_field_found);

  if is_all_required_fileds_found == false { return Err(NotAllRequiredFieldsFound); }

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
  use super::super::{FIXMessage, FIXMessageField, FIX_MESSAGE_DELIMITER};
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
  fn it_should_complain_when_checksum_format_is_invalid() {
    let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
      "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=2ZZ"];
    let message: String = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));

    assert_eq!(parse(&message).unwrap_err(),
      InvalidChecksum(ChecksumFieldInvalidFormat("2ZZ".parse::<u32>().unwrap_err())));
  }

  #[test]
  fn it_should_complain_when_checksum_is_invalid() {
    let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
      "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=231"];
    let message: String = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));

    assert_eq!(parse(&message).unwrap_err(), InvalidChecksumValue);
  }

  #[test]
  fn it_should_complain_when_invalid_field_structure_found() {
    let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=", "49=BRKR", "56=INVMGR",
      "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=188"];
    let message: String = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));

    assert_eq!(parse(&message).unwrap_err(), InvalidFieldStructure);
  }

  #[test]
  fn it_should_complain_when_the_first_field_is_incorrect() {
    let message_parts: Vec<&str> = vec!["9=FIX.4.2", "8=73", "35=0", "49=BRKR", "56=INVMGR",
      "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=236"];
    let message: String = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));

    assert_eq!(parse(&message).unwrap_err(), InvalidFirstField("9"));
  }

  #[test]
  fn it_should_complain_when_the_second_field_is_incorrect() {
    let message_parts: Vec<&str> = vec!["8=FIX.4.2", "35=73", "9=0", "49=BRKR", "56=INVMGR",
      "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=236"];
    let message: String = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));

    assert_eq!(parse(&message).unwrap_err(), InvalidSecondField("35"));
  }

  #[test]
  fn it_should_complain_when_the_third_field_is_incorrect() {
    let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "34=0", "49=BRKR", "56=INVMGR",
      "35=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=236"];
    let message: String = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));

    assert_eq!(parse(&message).unwrap_err(), InvalidThirdField("34"));
  }

  #[test]
  fn it_should_complain_when_not_all_required_header_fields_presented() {
    let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
      "52=19980604-07:58:28", "112=19980604-07:58:28", "10=173"];
    let message: String = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));

    assert_eq!(parse(&message).unwrap_err(), NotAllRequiredFieldsFound);
  }

  #[test]
  fn it_should_parse_fix_message() {
    let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
      "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=236"];
    let message: String = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));

    let fix_message = FIXMessage {
      version: "FIX".to_string(),
      data: vec![
        FIXMessageField { tag: "8".to_string(), value: "FIX.4.2".to_string() } ,
        FIXMessageField { tag: "9".to_string(), value: "73".to_string() } ,
        FIXMessageField { tag: "35".to_string(), value: "0".to_string() } ,
        FIXMessageField { tag: "49".to_string(), value: "BRKR".to_string() } ,
        FIXMessageField { tag: "56".to_string(), value: "INVMGR".to_string() } ,
        FIXMessageField { tag: "34".to_string(), value: "235".to_string() } ,
        FIXMessageField { tag: "52".to_string(), value: "19980604-07:58:28".to_string() } ,
        FIXMessageField { tag: "112".to_string(), value: "19980604-07:58:28".to_string() } ,
        FIXMessageField { tag: "10".to_string(), value: "236".to_string() }
      ]
    };

    assert_eq!(parse(&message).unwrap(), fix_message);
  }
}
