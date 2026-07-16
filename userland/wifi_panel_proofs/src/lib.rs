// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the WiFi settings panel. The real panel and wire source are
//! included and driven through the full detection and selection flow without a
//! device or a renderer.

extern crate alloc;

#[path = "../../capsule_settings/src/wifi/mod.rs"]
pub mod wifi;

#[cfg(test)]
mod panel_tests;
