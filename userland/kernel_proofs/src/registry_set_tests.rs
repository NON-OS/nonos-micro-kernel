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

//! What enters the set, what leaves it, and the one byte the reader decodes.

use crate::registry_support::{record, TABLE_IN_USE};
use crate::security::attest_registry::{
    forget_attested, registry_entries, registry_root, AttestedCapsule, ENTRY_LEN,
};
use crate::security::dev_roots::Authority;

#[test]
fn a_capsule_that_exited_leaves_the_set() {
    let _guard = TABLE_IN_USE.lock().unwrap();
    let before = registry_root();
    record(9021, 0x33, 4, Authority::Vendor);
    assert_ne!(registry_root(), before);
    forget_attested(9021);
    assert_eq!(registry_root(), before);
    let gone = registry_entries();
    assert!(!gone.chunks_exact(ENTRY_LEN).any(|e| e[..4] == 9021u32.to_be_bytes()));
}

#[test]
fn forgetting_a_pid_that_was_never_recorded_is_not_an_error() {
    let _guard = TABLE_IN_USE.lock().unwrap();
    let before = registry_root();
    forget_attested(9999);
    assert_eq!(registry_root(), before);
}

#[test]
fn the_entry_carries_the_mask_the_kernel_granted() {
    let _guard = TABLE_IN_USE.lock().unwrap();
    record(9031, 0x44, 0x183D, Authority::Vendor);
    let entries = registry_entries();
    let mine = entries
        .chunks_exact(ENTRY_LEN)
        .find(|e| e[..4] == 9031u32.to_be_bytes())
        .expect("recorded capsule is in the set");
    assert_eq!(u64::from_be_bytes(mine[36..44].try_into().unwrap()), 0x183D);
    assert_eq!(&mine[4..36], &[0x44; 32]);
    forget_attested(9031);
}

#[test]
fn the_authority_byte_is_what_the_reader_decodes() {
    let entry = |authority| AttestedCapsule { pid: 1, measurement: [0; 32], caps: 0, authority };
    assert_eq!(entry(Authority::Vendor).digest_input()[44], 0);
    assert_eq!(entry(Authority::Developer(3)).digest_input()[44], 4);
    assert_eq!(entry(Authority::Publisher).digest_input()[44], 255);
}
