extern crate fix_message;
extern crate fix_checksum;

use fix_message::*;
use fix_checksum::FIXChecksumValidatorError::*;

// Message parser ==============================================================

#[test]
fn parser_should_complain_when_checksum_not_found() {
  let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
    "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28"];
  let message: String = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));

  assert_eq!(parse(&message).unwrap_err(), InvalidChecksum(ChecksumFieldNotFound));
}

#[test]
fn parser_should_complain_when_checksum_format_is_invalid() {
  let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
    "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=2ZZ"];
  let message: String = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));

  assert_eq!(parse(&message).unwrap_err(),
    InvalidChecksum(ChecksumFieldInvalidFormat("2ZZ".parse::<u32>().unwrap_err())));
}

#[test]
fn parser_should_complain_when_checksum_is_invalid() {
  let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
    "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=231"];
  let message: String = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));

  assert_eq!(parse(&message).unwrap_err(), InvalidChecksumValue);
}

#[test]
fn parser_should_complain_when_invalid_field_structure_found() {
  let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=", "49=BRKR", "56=INVMGR",
    "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=188"];
  let message: String = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));

  assert_eq!(parse(&message).unwrap_err(), InvalidFieldStructure);
}

#[test]
fn parser_should_complain_when_the_first_field_is_incorrect() {
  let message_parts: Vec<&str> = vec!["9=FIX.4.2", "8=73", "35=0", "49=BRKR", "56=INVMGR",
    "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=236"];
  let message: String = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));

  assert_eq!(parse(&message).unwrap_err(), InvalidFirstField("9"));
}

#[test]
fn parser_should_complain_when_the_second_field_is_incorrect() {
  let message_parts: Vec<&str> = vec!["8=FIX.4.2", "35=73", "9=0", "49=BRKR", "56=INVMGR",
    "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=236"];
  let message: String = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));

  assert_eq!(parse(&message).unwrap_err(), InvalidSecondField("35"));
}

#[test]
fn parser_should_complain_when_the_third_field_is_incorrect() {
  let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "34=0", "49=BRKR", "56=INVMGR",
    "35=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=236"];
  let message: String = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));

  assert_eq!(parse(&message).unwrap_err(), InvalidThirdField("34"));
}

#[test]
fn parser_should_complain_when_not_all_required_header_fields_presented() {
  let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
    "52=19980604-07:58:28", "112=19980604-07:58:28", "10=173"];
  let message: String = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));

  assert_eq!(parse(&message).unwrap_err(), NotAllRequiredFieldsFound);
}

#[test]
fn parser_should_complain_when_required_header_field_repeated() {
  let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "35=0", "49=BRKR", "56=INVMGR",
    "52=19980604-07:58:28", "112=19980604-07:58:28", "10=131"];
  let message: String = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));

  assert_eq!(parse(&message).unwrap_err(), ExtraRequiredFieldFound);
}

#[test]
fn parser_should_parse_fix_message() {
  let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
    "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=236"];
  let message: String = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));

  let expected_fix_message = FIXMessage {
    version: "FIX.4.2".to_string(),
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

  assert_eq!(parse(&message).unwrap(), expected_fix_message);
}
