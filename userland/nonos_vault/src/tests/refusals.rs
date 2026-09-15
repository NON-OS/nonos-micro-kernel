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

//! What is refused rather than misread.

use crate::blob::OVERHEAD;
use crate::{open, seal, subkey};

use super::*;

#[test]
fn a_v1_record_is_refused_rather_than_misread() {
    /*
     * Version one sealed under the machine key directly, with no per-record
     * key. Opening it under these rules would use the wrong key anyway; it
     * has to fail on the version, before any derivation.
     */
    let mut blob = sealed(b"settings.theme", b"dark");
    blob[8] = 1;
    let mut out = [0u8; 4];
    assert!(open(&ROOT, b"settings.theme", &blob, &mut out).is_err());
}

#[test]
fn anything_that_is_not_a_record_is_refused() {
    let mut out = [0u8; 8];
    assert!(open(&ROOT, b"x", &[], &mut out).is_err());
    assert!(open(&ROOT, b"x", &[0u8; OVERHEAD], &mut out).is_err());
    assert!(open(&ROOT, b"x", &[0xFFu8; 64], &mut out).is_err());
}

#[test]
fn an_empty_or_overlong_record_name_is_refused() {
    assert!(subkey(&ROOT, b"").is_err());
    let long = [b'x'; crate::MAX_RECORD + 1];
    assert!(subkey(&ROOT, &long).is_err());
    let at_cap = [b'x'; crate::MAX_RECORD];
    assert!(subkey(&ROOT, &at_cap).is_ok(), "exactly the cap must fit");
}

#[test]
fn a_dead_entropy_source_seals_nothing() {
    /*
     * An all-zero nonce is what a stuck source returns, and two records
     * sealed with it under one key lose the key. Refusing costs a retry.
     */
    let mut out = alloc::vec![0u8; 4 + OVERHEAD];
    assert!(seal(&ROOT, b"settings.theme", b"dark", &[0u8; 12], &mut out).is_err());
}

#[test]
fn a_short_output_buffer_is_refused_rather_than_truncated() {
    let mut out = alloc::vec![0u8; OVERHEAD];
    assert!(seal(&ROOT, b"x", b"too long for this", &NONCE, &mut out).is_err());
}
