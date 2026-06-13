// NONOS std PAL: OS error decoding.
//
// NONOS syscalls do not expose a C-style errno; failures surface as
// negative return codes handled at each call site. There is no ambient
// errno, so this is intentionally minimal.

use crate::io::ErrorKind;
use crate::string::String;

pub type RawOsError = i32;

pub fn errno() -> RawOsError {
    0
}

pub fn is_interrupted(_code: RawOsError) -> bool {
    false
}

pub fn decode_error_kind(_code: RawOsError) -> ErrorKind {
    ErrorKind::Uncategorized
}

pub fn error_string(_errno: RawOsError) -> String {
    String::from("nonos os error")
}
