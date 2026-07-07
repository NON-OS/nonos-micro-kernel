// NONOS Operating System (AGPL-3.0-or-later)
#[path = "../../../capsule_driver_usb_hid/src/descriptors/types.rs"]
pub mod types;
#[path = "../../../capsule_driver_usb_hid/src/descriptors/binding.rs"]
pub mod binding;
// The parser returns Result<_, ()>, its own choice.
#[allow(clippy::result_unit_err)]
#[path = "../../../capsule_driver_usb_hid/src/descriptors/parse.rs"]
pub mod parse;
pub use parse::hid_bindings;
