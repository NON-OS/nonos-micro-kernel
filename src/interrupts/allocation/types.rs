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

use crate::arch::trap::contract::TrapFrame;

/// A handler for a vector that arrives without an error code.
///
/// Taken as a trait object over the portable frame rather than over one
/// architecture's concrete one. This named the `x86_64` crate's
/// `InterruptStackFrame`, which put a PC's stack layout into the signature of
/// every handler the kernel can register.
pub type NoErrorHandler = fn(&dyn TrapFrame);

/// A handler for a vector that arrives with an error code pushed behind it.
pub type ErrorCodeHandler = fn(&dyn TrapFrame, u64);

pub const VECTOR_COUNT: usize = 256;
pub const RESERVED_VECTORS_END: u8 = 32;
pub const TIMER_VECTOR: u8 = 32;
pub const KEYBOARD_VECTOR: u8 = 33;
pub const SYSCALL_VECTOR: u8 = 0x80;
