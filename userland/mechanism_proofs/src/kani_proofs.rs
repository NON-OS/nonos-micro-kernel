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

//! Kani harnesses: the buddy address arithmetic holds for every input, not just
//! the sampled ones.

use crate::buddy::constants::helpers::{buddy_address, order_to_size};

// The buddy of the buddy of a block is the block itself: the address XOR is an
// involution, for every address and every order.
#[kani::proof]
fn buddy_address_is_an_involution() {
    let addr: u64 = kani::any();
    let order: usize = kani::any();
    kani::assume(order < 64);
    assert_eq!(buddy_address(buddy_address(addr, order), order), addr);
}

// Splitting a block halves its size exactly: an order k block is two order k-1
// blocks, so no memory is created or lost by a split.
#[kani::proof]
fn a_split_conserves_size() {
    let k: usize = kani::any();
    kani::assume(k < 62);
    assert_eq!(order_to_size(k + 1), 2 * order_to_size(k));
}
