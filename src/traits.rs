use super::{FIXMessageResult, FIXMessageField};

pub trait MessageValidator<T> {
  fn validate_fields_order(&self) -> FIXMessageResult<T>;
}

impl <'a> MessageValidator<(&'a str, &'a str)> for FIXMessageResult<'a, (usize, &'a str, &'a str)> {
  fn validate_fields_order(&self) -> FIXMessageResult<(&'a str, &'a str)> {
    unimplemented!();
  }
}

// impl <'a> MessageValidator<Vec<FIXMessageField>> for Vec<&'a str> {
//   fn validate_fields_order(&self) -> FIXMessageResult<Vec<FIXMessageField>> {
//     unimplemented!();
//   }
// }

// impl <'a> MessageValidator<Vec<&'a str>> for Vec<FIXMessageField> {
//   fn validate(&self) -> FIXMessageResult<Vec<&'a str>> {
//     unimplemented!();
//   }
// }

// pub trait MessageMapper<T> {
//   fn map_message_fields(&self) -> FIXMessageResult<T>;
// }

// impl <'a> MessageMapper<FIXMessageField> for FIXMessageResult<'a, (&'a str, &'a str)> {
//   fn map_message_fields(&self) -> FIXMessageResult<FIXMessageField> {
//     self.and_then(|(tag, value)| Ok(FIXMessageField {tag: tag.to_string(), value: value.to_string()}))
//   }
// }
