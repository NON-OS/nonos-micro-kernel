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

//! The reader folds a byte string; the kernel folds entry by entry. Every
//! claim a receipt makes rests on those reaching the same value.

use super::fixtures::{flatten, kernel_fold, three_capsules};
use crate::kernel::DOMAIN;
use crate::root::fold_root;

#[test]
fn the_reader_reaches_the_value_the_machine_folded() {
    let set = three_capsules();
    assert_eq!(fold_root(&flatten(&set)), kernel_fold(&set));
}

#[test]
fn an_empty_registry_is_a_statement_and_not_an_absence() {
    assert_eq!(fold_root(&[]), kernel_fold(&[]));
    assert_ne!(fold_root(&[]), [0u8; 32]);
}

#[test]
fn the_count_is_bound_and_not_merely_carried() {
    let bytes = flatten(&three_capsules());
    let mut hasher = blake3::Hasher::new();
    hasher.update(DOMAIN);
    hasher.update(&1u32.to_be_bytes());
    hasher.update(&bytes);
    assert_ne!(fold_root(&bytes), *hasher.finalize().as_bytes());
}

#[test]
fn the_domain_separates_the_root_from_a_bare_digest() {
    let bytes = flatten(&three_capsules());
    assert_ne!(fold_root(&bytes), *blake3::hash(&bytes).as_bytes());
}
