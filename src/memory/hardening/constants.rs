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

//! Memory Hardening Constants

// The control-register bits are defined once, by the module that writes them.
// A second definition here is how the check ends up testing a different bit
// than the bring-up set.
pub use crate::memory::mmu::CR4_REQUIRED_BITS;

/// Pattern used for heap corruption detection.
pub const CORRUPTION_PATTERN: u64 = 0xDEADBEEFCAFEBABE;

/// Canary mixing constant for stack protection.
pub const CANARY_MIX_CONSTANT: u64 = 0x9e3779b97f4a7c15;

/// NOP instruction byte for checking suspicious code.
pub const NOP_INSTRUCTION: u8 = 0x90;

/// Bytes to check for suspicious NOP sleds.
pub const NOP_SLED_CHECK_SIZE: usize = 16;
