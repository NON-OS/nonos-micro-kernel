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

//! The entries the kernel hands out fold to the root the kernel signs.

use crate::registry_support::{record, refold, TABLE_IN_USE};
use crate::security::attest_registry::{
    forget_attested, registry_entries, registry_root, ENTRY_LEN,
};
use crate::security::dev_roots::Authority;

#[test]
fn the_entries_fold_to_the_root() {
    let _guard = TABLE_IN_USE.lock().unwrap();
    let before = registry_root();
    record(9001, 0xAA, 0x39, Authority::Vendor);
    record(9002, 0xBB, 0x7919, Authority::Developer(1));
    record(9003, 0xCC, 0x183D, Authority::Publisher);
    let entries = registry_entries();
    assert_eq!(entries.len() % ENTRY_LEN, 0);
    assert_eq!(refold(&entries), registry_root());
    for pid in [9001, 9002, 9003] {
        forget_attested(pid);
    }
    assert_eq!(registry_root(), before, "the table must be left as it was found");
}

#[test]
fn the_order_capsules_started_in_does_not_change_the_root() {
    let _guard = TABLE_IN_USE.lock().unwrap();
    record(9011, 0x11, 1, Authority::Vendor);
    record(9012, 0x22, 2, Authority::Vendor);
    let forward = registry_root();
    forget_attested(9011);
    forget_attested(9012);
    record(9012, 0x22, 2, Authority::Vendor);
    record(9011, 0x11, 1, Authority::Vendor);
    assert_eq!(registry_root(), forward);
    forget_attested(9011);
    forget_attested(9012);
}

#[test]
fn an_empty_registry_still_folds_the_domain_and_count() {
    let _guard = TABLE_IN_USE.lock().unwrap();
    if registry_entries().is_empty() {
        assert_eq!(registry_root(), refold(&[]));
        assert_ne!(registry_root(), [0u8; 32]);
    }
}
