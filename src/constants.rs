/// Standard FIX message delimiter. All fields in a FIX message are terminated
/// by a delimiter character (code `0x01`)
pub const FIX_MESSAGE_DELIMITER: char = '\x01';

/// Standard FIX message field tag-value delimiter (code `0x3D`, `=`)
pub const FIX_MESSAGE_FIELD_DELIMITER: char = '\x3D';
