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

// The real buddy-allocator order and size constants and the pure arithmetic the
// allocator runs: order-to-size, size-to-order, and the buddy address XOR.
// Reconstructed as a module tree so `helpers.rs` resolves its `super::orders`
// and `super::sizes` imports against the real files.

#[allow(dead_code)]
#[path = "../../../../../src/memory/buddy_alloc/constants/orders.rs"]
pub mod orders;

#[allow(dead_code)]
#[path = "../../../../../src/memory/buddy_alloc/constants/sizes.rs"]
pub mod sizes;

#[allow(dead_code)]
#[path = "../../../../../src/memory/buddy_alloc/constants/helpers.rs"]
pub mod helpers;
