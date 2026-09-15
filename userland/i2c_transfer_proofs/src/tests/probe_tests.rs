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

//! The probes setup uses to find the touchpad among the firmware's
//! candidate addresses, against a bus where one of them is real.

use nonos_i2cmodel::Config;

use super::fixture::{bench, core, PAD};
use crate::transaction::{probe, probe_hid};

#[test]
fn the_hid_probe_recognises_a_device_by_its_descriptor_length() {
    let b = bench(Config::LPSS);
    assert_eq!(probe_hid(&b.driver, PAD).ok(), Some(true));
    assert!(core(|c| c.violations().is_empty()), "{:?}", core(|c| c.violations().to_vec()));
}

#[test]
fn the_hid_probe_says_no_for_an_address_nobody_answers() {
    /*
     * A NACK is the normal outcome of probing a candidate that is not
     * fitted, not an error: setup moves on to the next address, and a probe
     * that reported it as a failure would abandon the controller.
     */
    let b = bench(Config::LPSS);
    assert_eq!(probe_hid(&b.driver, 0x2C).ok(), Some(false));
}

#[test]
fn the_bare_probe_finds_the_same_device_with_a_single_read() {
    let b = bench(Config::LPSS);
    assert_eq!(probe(&b.driver, PAD).ok(), Some(true));
    assert_eq!(probe(&b.driver, 0x2C).ok(), Some(false));
}
