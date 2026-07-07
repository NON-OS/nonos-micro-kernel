// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the USB HID descriptor parser over untrusted
//! device-controlled configuration descriptors.

extern crate alloc;

pub mod descriptors;
pub mod protocol;

#[cfg(test)]
mod usb_tests;
