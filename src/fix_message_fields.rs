//! This module contains a list of all standard FIX message fields

// Standard Header fields ======================================================
/// `8 - BeginString`
pub const BEGIN_STRING: &'static str = "8";

/// `9 - BodyLength`
pub const BODY_LENGTH: &'static str = "9";

/// `35 - MsgType`
pub const MSG_TYPE: &'static str = "35";

/// `49 - SenderCompID`
pub const SENDER_COMP_ID: &'static str = "49";

/// `56 - TargetCompID`
pub const TARGET_COMP_ID: &'static str = "56";

/// `34 - MsgSeqNum`
pub const MSG_SEQ_NUM: &'static str = "34";

/// `52 - SendingTime`
pub const SENDING_TIME: &'static str = "52";


// Standard Trailer fields =====================================================
/// `10 - CheckSum`
pub const CHECK_SUM: &'static str = "10";
