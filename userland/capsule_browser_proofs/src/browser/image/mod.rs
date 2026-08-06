// NONOS Operating System (AGPL-3.0-or-later)
// The decoder under test is pub(super), so it is only pulled in for the tests
// that exercise it; that keeps the non-test build free of an unused import.
#[cfg(test)]
#[path = "../../../../base64/src/decode.rs"]
pub mod base64;

#[cfg(test)]
mod base64_tests;
