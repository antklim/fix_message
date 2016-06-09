use super::{
  FIXMessageResult, FIXMessage, FIXMessageField, FIXMessageError,
  FIX_MESSAGE_DELIMITER, FIX_MESSAGE_FIELD_DELIMITER
};
use super::FIXMessageError::*;
use super::fix_message_fields::*;

use fix_checksum::validate as validate_checksum;
use fix_checksum::FIXChecksumValidatorError;

fn validate_field_structure<'a>((index, field): (usize, &&'a str)) -> FIXMessageResult<'a, (usize, &'a str, &'a str)> {
  let field_parts = (*field).splitn(2, FIX_MESSAGE_FIELD_DELIMITER).collect::<Vec<&str>>();
  let (tag, value) = (field_parts[0], field_parts[1]);
  if tag == "" || value == "" { return Err(InvalidFieldStructure) }
  Ok((index, tag, value))
}

fn validate_fields_order<'a>(res: FIXMessageResult<'a, (usize, &'a str, &'a str)>) -> FIXMessageResult<'a, (&'a str, &'a str)> {
  res.and_then(|(index, tag, value)| {
    let tag_to_check: Option<(&str, FIXMessageError)> = match index {
      0 => Some((BEGIN_STRING, InvalidFirstField(tag))),
      1 => Some((BODY_LENGTH, InvalidSecondField(tag))),
      2 => Some((MSG_TYPE, InvalidThirdField(tag))),
      _ => None,
    };

    tag_to_check
      .map_or(Ok((tag, value)), |(expected_tag, err)| {
        if tag != expected_tag { Err(err) }
        else { Ok((tag, value)) }
      })
  })
}

fn map_to_fix_message_field<'a>(res: FIXMessageResult<'a, (&'a str, &'a str)>) -> FIXMessageResult<'a, FIXMessageField> {
  res.and_then(|(tag, value)| Ok(FIXMessageField {tag: tag.to_string(), value: value.to_string()}))
}

fn is_required_field(tag: &str) -> bool {
  match tag {
    BEGIN_STRING |
    BODY_LENGTH |
    MSG_TYPE |
    SENDER_COMP_ID |
    TARGET_COMP_ID |
    MSG_SEQ_NUM |
    SENDING_TIME => true,
    _ => false,
  }
}

fn required_fields<'a>() -> Vec<&'a str> {
  vec![BEGIN_STRING, BODY_LENGTH, MSG_TYPE, SENDER_COMP_ID, TARGET_COMP_ID, MSG_SEQ_NUM, SENDING_TIME]
}

fn validate_and_parse<'a>(message_fields: Vec<&'a str>) -> FIXMessageResult<'a, Vec<FIXMessageField>> {
  let mut required_fields= required_fields();
  message_fields.iter().enumerate()
    .map(validate_field_structure)
    .map(validate_fields_order)
    .map(|res: FIXMessageResult<'a, (&'a str, &'a str)>|
      res.and_then(|(tag, value)| {
        if is_required_field(tag) {
          let len_before_retain = required_fields.len();
          required_fields.retain(|&element| element != tag);
          if required_fields.len() != len_before_retain - 1 { return Err(ExtraRequiredFieldFound) }
        }

        if tag == CHECK_SUM {
          if !required_fields.is_empty() { return Err(NotAllRequiredFieldsFound) }
          return Ok((tag, value))
        }

        return Ok((tag, value))
      })
    )
    .map(map_to_fix_message_field)
    .collect::<FIXMessageResult<Vec<FIXMessageField>>>()
}

/// This function validates and parses FIX message
///
/// # Errors
/// Returns `ChecksumFieldNotFound` when `10 - ChekSum` field not found in message:
///
/// ```
/// extern crate fix_checksum;
///
/// # #[macro_use] extern crate fix_message;
/// # fn main() {
/// use fix_message::*;
/// use fix_checksum::FIXChecksumValidatorError::*;
///
/// let mut message_parts = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
///   "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28"];
/// let mut message = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));
///
/// assert_eq!(parse(&message).unwrap_err(), InvalidChecksum(ChecksumFieldNotFound));
/// # }
/// ```
///
/// Returns `ChecksumFieldInvalidFormat` when `10 - ChekSum` field value in invalid format:
///
/// ```
/// extern crate fix_checksum;
///
/// # #[macro_use] extern crate fix_message;
/// # fn main() {
/// use fix_message::*;
/// use fix_checksum::FIXChecksumValidatorError::*;
///
/// let message_parts = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
///   "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=2ZZ"];
/// let message = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));
///
/// assert_eq!(parse(&message).unwrap_err(),
///   InvalidChecksum(ChecksumFieldInvalidFormat("2ZZ".parse::<u32>().unwrap_err())));
/// # }
/// ```
///
/// Returns `InvalidChecksumValue` when `10 - ChekSum` field value is not valid:
///
/// ```
/// # #[macro_use] extern crate fix_message;
/// # fn main() {
/// use fix_message::*;
///
/// let message_parts = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
///   "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=231"];
/// let message = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));
///
/// assert_eq!(parse(&message).unwrap_err(), InvalidChecksumValue);
/// # }
/// ```
///
/// Returns `InvalidFieldStructure` when FIX message field does not have value:
///
/// ```
/// # #[macro_use] extern crate fix_message;
/// # fn main() {
/// use fix_message::*;
///
/// let message_parts = vec!["8=FIX.4.2", "9=73", "35=", "49=BRKR", "56=INVMGR",
///   "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=188"];
/// let message = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));
///
/// assert_eq!(parse(&message).unwrap_err(), InvalidFieldStructure);
/// # }
/// ```
///
/// Returns `InvalidFirstField` when the first field in message is not `8 - BeginString`:
///
/// ```
/// # #[macro_use] extern crate fix_message;
/// # fn main() {
/// use fix_message::*;
///
/// let message_parts = vec!["9=FIX.4.2", "8=73", "35=0", "49=BRKR", "56=INVMGR",
///   "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=236"];
/// let message = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));
///
/// assert_eq!(parse(&message).unwrap_err(), InvalidFirstField("9"));
/// # }
/// ```
///
/// Returns `InvalidSecondField` when the second field in message is not `9 - BodyLength`:
///
/// ```
/// # #[macro_use] extern crate fix_message;
/// # fn main() {
/// use fix_message::*;
///
/// let message_parts = vec!["8=FIX.4.2", "35=73", "9=0", "49=BRKR", "56=INVMGR",
///   "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=236"];
/// let message = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));
///
/// assert_eq!(parse(&message).unwrap_err(), InvalidSecondField("35"));
/// # }
/// ```
///
/// Returns `InvalidThirdField` when the third field in message is not `35 - MsgType`:
///
/// ```
/// # #[macro_use] extern crate fix_message;
/// # fn main() {
/// use fix_message::*;
///
/// let message_parts = vec!["8=FIX.4.2", "9=73", "34=0", "49=BRKR", "56=INVMGR",
///   "35=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=236"];
/// let message = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));
///
/// assert_eq!(parse(&message).unwrap_err(), InvalidThirdField("34"));
/// # }
/// ```
///
/// Returns `NotAllRequiredFieldsFound` when not all fields required for standard
/// header were found:
///
/// ```
/// # #[macro_use] extern crate fix_message;
/// # fn main() {
/// use fix_message::*;
///
/// let message_parts = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
///   "52=19980604-07:58:28", "112=19980604-07:58:28", "10=173"];
/// let message = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));
///
/// assert_eq!(parse(&message).unwrap_err(), NotAllRequiredFieldsFound);
/// # }
/// ```
///
/// Returns `ExtraRequiredFieldFound` when additional required field has been
/// found in standard header:
///
/// ```
/// # #[macro_use] extern crate fix_message;
/// # fn main() {
/// use fix_message::*;
///
/// let message_parts = vec!["8=FIX.4.2", "9=73", "35=0", "35=0", "49=BRKR", "56=INVMGR",
///   "52=19980604-07:58:28", "112=19980604-07:58:28", "10=131"];
/// let message = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));
///
/// assert_eq!(parse(&message).unwrap_err(), ExtraRequiredFieldFound);
/// # }
/// ```
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # #[macro_use] extern crate fix_message;
/// # fn main() {
/// use fix_message::*;
///
/// let message_parts = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
///   "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=236"];
/// let message = message_parts.join(&(FIX_MESSAGE_DELIMITER.to_string()));
///
/// let expected_fix_message = FIXMessage {
///   version: "FIX.4.2".to_string(),
///   data: vec![
///     FIXMessageField { tag: "8".to_string(), value: "FIX.4.2".to_string() } ,
///     FIXMessageField { tag: "9".to_string(), value: "73".to_string() } ,
///     FIXMessageField { tag: "35".to_string(), value: "0".to_string() } ,
///     FIXMessageField { tag: "49".to_string(), value: "BRKR".to_string() } ,
///     FIXMessageField { tag: "56".to_string(), value: "INVMGR".to_string() } ,
///     FIXMessageField { tag: "34".to_string(), value: "235".to_string() } ,
///     FIXMessageField { tag: "52".to_string(), value: "19980604-07:58:28".to_string() } ,
///     FIXMessageField { tag: "112".to_string(), value: "19980604-07:58:28".to_string() } ,
///     FIXMessageField { tag: "10".to_string(), value: "236".to_string() }
///   ]
/// };
///
///  assert_eq!(parse(&message).unwrap(), expected_fix_message);
/// # }
/// ```
pub fn parse(inbound_message: &str) -> FIXMessageResult<FIXMessage> {
  validate_checksum(inbound_message)
    .map_err(|err: FIXChecksumValidatorError| InvalidChecksum(err))
    .and_then(|is_valid_value: bool| {
      if !is_valid_value { return Err(InvalidChecksumValue) }
      Ok(inbound_message.split(FIX_MESSAGE_DELIMITER).collect::<Vec<&str>>())
    })
    .and_then(validate_and_parse)
    .and_then(|fix_message_fields: Vec<FIXMessageField>|
      Ok(FIXMessage {
        version: fix_message_fields[0].value.clone(),
        data: fix_message_fields
      })
    )
}

#[cfg(test)]
mod tests;
