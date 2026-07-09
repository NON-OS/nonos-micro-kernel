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

//! Kani harnesses: the constant-time primitives are functionally correct for
//! every input, not just the sampled ones.

use crate::crypto::constant_time::{
    ct_eq_32, ct_eq_u64, ct_gt_u64, ct_lt_u64, ct_select_u64, ct_select_u64_bit,
};

#[kani::proof]
fn ct_eq_32_equals_array_equality() {
    let a: [u8; 32] = kani::any();
    let b: [u8; 32] = kani::any();
    assert_eq!(ct_eq_32(&a, &b), a == b);
}

#[kani::proof]
fn ct_integer_comparators_are_correct() {
    let a: u64 = kani::any();
    let b: u64 = kani::any();
    assert_eq!(ct_eq_u64(a, b) != 0, a == b);
    assert_eq!(ct_lt_u64(a, b) != 0, a < b);
    assert_eq!(ct_gt_u64(a, b) != 0, a > b);
}

#[kani::proof]
fn ct_select_is_correct() {
    let cond: bool = kani::any();
    let a: u64 = kani::any();
    let b: u64 = kani::any();
    assert_eq!(ct_select_u64(cond, a, b), if cond { a } else { b });
    assert_eq!(ct_select_u64_bit(1, a, b), a);
    assert_eq!(ct_select_u64_bit(0, a, b), b);
}
