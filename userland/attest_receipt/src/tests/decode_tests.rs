// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The masks below are real, from a v0.9.2 boot. The claim the system makes
//! about a wallet is a claim about one bit of one of them.

use super::fixtures::{BROWSER, COMPOSITOR, KEYRING};
use crate::decode::{capability_names, reaches_network, Authority};
use crate::kernel::capability::Capability;

fn grants(mask: u64, c: Capability) -> bool {
    capability_names(mask).contains(&c.as_str().to_string())
}

#[test]
fn the_keyring_holding_the_wallet_key_cannot_reach_the_network() {
    assert!(!reaches_network(KEYRING));
    assert!(grants(KEYRING, Capability::Crypto));
    assert!(grants(KEYRING, Capability::IPC));
}

#[test]
fn the_compositor_that_sees_every_window_cannot_reach_the_network() {
    assert!(!reaches_network(COMPOSITOR));
    assert!(grants(COMPOSITOR, Capability::GraphicsDisplayQuery));
}

#[test]
fn the_browser_can_and_the_receipt_says_so() {
    // Without this the two above would pass on a decoder that lost the bit.
    assert!(reaches_network(BROWSER));
}

#[test]
fn an_undefined_bit_is_reported_rather_than_dropped() {
    let names = capability_names(KEYRING | 1 << 63);
    assert!(names.iter().any(|n| n.contains("unrecognised")), "{names:?}");
}

#[test]
fn an_empty_mask_names_nothing() {
    assert!(capability_names(0).is_empty());
}

#[test]
fn every_defined_capability_is_named_when_the_mask_is_full() {
    let all = Capability::all();
    let full = all.iter().fold(0u64, |acc, c| acc | c.bit());
    assert_eq!(capability_names(full).len(), all.len());
}

#[test]
fn a_slot_past_this_readers_table_is_still_reported() {
    assert!(Authority::from_byte(200).describe().contains("beyond this reader's table"));
    assert!(Authority::from_byte(255).describe().contains("no proof"));
    assert_eq!(Authority::from_byte(2).describe(), "developer key 1");
}
