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

//! A set that reads well, next to a genuine signature over a different one.

use super::fixtures::{entry, flatten, kernel_fold, three_capsules, KEYRING};
use crate::kernel::capability::Capability;
use crate::root::verify_root;

#[test]
fn widening_a_capability_mask_is_caught() {
    let honest = three_capsules();
    let signed = kernel_fold(&honest);
    let mut tampered = honest.clone();
    tampered[0] = entry(12, 0xAA, KEYRING | Capability::Network.bit(), 0);
    assert!(verify_root(&flatten(&tampered), signed).is_err());
    assert!(verify_root(&flatten(&honest), signed).is_ok());
}

#[test]
fn dropping_a_capsule_from_the_set_is_caught() {
    let honest = three_capsules();
    let signed = kernel_fold(&honest);
    assert!(verify_root(&flatten(&honest[..2]), signed).is_err());
}

#[test]
fn reordering_the_set_is_caught() {
    let honest = three_capsules();
    let signed = kernel_fold(&honest);
    let mut swapped = honest.clone();
    swapped.swap(0, 2);
    assert!(verify_root(&flatten(&swapped), signed).is_err());
}

#[test]
fn the_same_measurement_under_a_different_authority_is_a_different_root() {
    let vendor = vec![entry(12, 0xAA, KEYRING, 0)];
    let developer = vec![entry(12, 0xAA, KEYRING, 1)];
    assert_ne!(kernel_fold(&vendor), kernel_fold(&developer));
}

#[test]
fn swapping_two_measurements_between_pids_is_caught() {
    let honest = three_capsules();
    let signed = kernel_fold(&honest);
    let swapped = vec![entry(12, 0xCC, KEYRING, 0), honest[1], entry(31, 0xAA, 0x183D, 0)];
    assert!(verify_root(&flatten(&swapped), signed).is_err());
}
