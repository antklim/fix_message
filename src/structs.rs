use std::fmt;

/// This structure represents field/value pair of FIX message
#[derive(PartialEq, Debug)]
pub struct FIXMessageField {
  /// A unique tag number of FIX message filed
  pub tag: String,
  /// A value associated with FIX message field
  pub value: String
}

impl fmt::Display for FIXMessageField {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}={}", self.tag, self.value)
  }
}

/// This structure represents the whole FIX message
///
/// ### Parsing FIX message
/// When parsing FIX messaged then field/value pairs stored in order they
/// were in the message.
///
/// ### Generating FIX messages
/// When generating FIX message then field/value pairs will be concatenated
/// in the order they stored in vector.
/// First two fileds (8, 9) should not be provided in data (will be ignored if
/// found). Field `8` - `BeginString` will be automatically added to the message
/// header and have `FIXMessage.version` value. Field `9` - `BodyLength` will be
/// automatically calculated and added to the message header.
#[derive(PartialEq, Debug)]
pub struct FIXMessage {
  /// FIX protocol version
  pub version: String,
  /// The list of field/value pairs stored in order they were in message
  pub data: Vec<FIXMessageField>
}

impl fmt::Display for FIXMessage {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}\n{:?}", self.version, self.data)
  }
}
