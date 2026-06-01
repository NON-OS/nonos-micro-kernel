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

pub const EXEC_RANDOMIZATION_RANGE: u64 = 0x4000_0000;
pub const STACK_RANDOMIZATION_RANGE: u64 = 0x0100_0000;
pub const HEAP_RANDOMIZATION_RANGE: u64 = 0x0200_0000;
pub(in crate::elf::aslr::manager) const LCG_MULTIPLIER: u64 = 6364136223846793005;
pub(in crate::elf::aslr::manager) const LCG_INCREMENT: u64 = 1;
pub(in crate::elf::aslr::manager) const FALLBACK_SEED: u64 = 0xDEAD_BEEF_CAFE_BABE;
