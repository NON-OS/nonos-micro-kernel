// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the WiFi settings panel. The real panel source is included
//! via #[path] and exercised against synthetic inputs: adapter discovery, the
//! untrusted scan-result parser, and the detect/select/enter-key/connect state
//! machine.

extern crate alloc;

#[path = "../../capsule_settings/src/wifi/interface.rs"]
pub mod interface;

/// The pure WiFi panel modules, grouped so their `super::` references resolve as
/// they do in the capsule. Adapter enumeration (`adapters`) is left out because
/// it calls the broker; the logic it drives (`interface::discover`) is proven.
pub mod wifi;

#[cfg(test)]
mod interface_tests;
#[cfg(test)]
mod panel_tests;
#[cfg(test)]
mod wire_tests;
