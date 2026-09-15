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

//! Which key a record gets, and what changes it.

use crate::{open, subkey};

use super::*;

#[test]
fn each_record_gets_a_different_key() {
    /*
     * The property the whole design rests on. If two records shared a key,
     * recovering one would open the other, and the separation this crate
     * advertises would be decoration.
     */
    let a = subkey(&ROOT, b"wallet.account").expect("a");
    let b = subkey(&ROOT, b"settings.theme").expect("b");
    assert_ne!(a, b);

    // Including names that differ by one byte, and by length alone.
    let c = subkey(&ROOT, b"settings.themf").expect("c");
    let d = subkey(&ROOT, b"settings.theme.").expect("d");
    assert_ne!(b, c);
    assert_ne!(b, d);
}

#[test]
fn the_same_record_always_derives_the_same_key() {
    let first = subkey(&ROOT, b"wallet.account").expect("first");
    for _ in 0..8 {
        assert_eq!(subkey(&ROOT, b"wallet.account").expect("again"), first);
    }
}

#[test]
fn a_different_root_derives_a_different_key() {
    // A different machine, or the same machine in a different boot state.
    let here = subkey(&ROOT, b"wallet.account").expect("here");
    let there = subkey(&OTHER_ROOT, b"wallet.account").expect("there");
    assert_ne!(here, there);
}

#[test]
fn a_record_does_not_open_under_another_name() {
    let blob = sealed(b"wallet.account", b"the seed phrase");
    let mut out = alloc::vec![0u8; 15];
    // Same length, so the header check passes and the tag is what refuses.
    assert!(open(&ROOT, b"wallet.accounx", &blob, &mut out).is_err());
    assert_eq!(out, alloc::vec![0u8; 15], "nothing is written on a failed open");
}

#[test]
fn a_record_does_not_open_on_another_machine() {
    let blob = sealed(b"wallet.account", b"the seed phrase");
    let mut out = alloc::vec![0u8; 15];
    assert!(open(&OTHER_ROOT, b"wallet.account", &blob, &mut out).is_err());
    assert_eq!(out, alloc::vec![0u8; 15]);
}
