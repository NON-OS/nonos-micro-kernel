// NONOS Operating System (AGPL-3.0-or-later)
// The real 802.11 frame layer from the iwlwifi driver, included so its pure
// IEEE encoding is checked exactly. `mgmt` resolves `super::header` against the
// real header file through this module.
#[path = "../../../capsule_driver_iwlwifi/src/dot11/header.rs"]
pub mod header;
#[path = "../../../capsule_driver_iwlwifi/src/dot11/mgmt.rs"]
pub mod mgmt;
