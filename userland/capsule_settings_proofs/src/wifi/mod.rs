// NONOS Operating System (AGPL-3.0-or-later)
//! Groups the pure WiFi panel source (network model, wire codec, panel state
//! machine) under one module so their `super::` references resolve exactly as in
//! the capsule, while the proofs drive them against synthetic inputs.

#[path = "../../../capsule_settings/src/wifi/network.rs"]
pub mod network;
// The connect state machine drives the connect-request encoder, which is
// test-only in the capsule, so the panel proofs compile it under test too.
#[cfg(test)]
#[path = "../../../capsule_settings/src/wifi/panel.rs"]
pub mod panel;
#[path = "../../../capsule_settings/src/wifi/wire.rs"]
pub mod wire;
