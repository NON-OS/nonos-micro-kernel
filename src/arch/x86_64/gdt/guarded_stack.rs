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

//! One stack with the page that catches its overflow.
//!
//! A stack grows down, so the guard is the field before it: the first address
//! past the end of the usable region, in the direction the stack pointer
//! moves. `percpu_guards` unmaps it once there is a paging manager able to,
//! and from then on running off the bottom faults instead of writing into
//! whatever the linker placed underneath.

use super::constants::DEFAULT_STACK_SIZE;

pub(super) const GUARD_BYTES: usize = 4096;

#[repr(C, align(4096))]
pub struct GuardedStack {
    /// Unmapped once paging is up. Overflow lands here and faults.
    pub guard: [u8; GUARD_BYTES],
    pub stack: [u8; DEFAULT_STACK_SIZE],
}

impl GuardedStack {
    pub const fn new() -> Self {
        Self { guard: [0; GUARD_BYTES], stack: [0; DEFAULT_STACK_SIZE] }
    }

    pub fn top(&self) -> u64 {
        self.stack.as_ptr() as u64 + DEFAULT_STACK_SIZE as u64
    }

    pub fn guard_base(&self) -> u64 {
        self.guard.as_ptr() as u64
    }
}
