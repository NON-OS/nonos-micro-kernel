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

extern crate alloc;

use alloc::vec::Vec;

use super::types::Capability;

#[inline]
pub fn caps_to_bits(caps: &[Capability]) -> u64 {
    caps.iter().fold(0u64, |acc, c| acc | c.bit())
}

#[inline]
pub fn bits_to_caps(bits: u64) -> Vec<Capability> {
    Capability::all().iter().copied().filter(|c| bits & c.bit() != 0).collect()
}

#[inline]
pub fn has_capability(bits: u64, cap: Capability) -> bool {
    bits & cap.bit() != 0
}

#[inline]
pub fn add_capability(bits: u64, cap: Capability) -> u64 {
    bits | cap.bit()
}

#[inline]
pub fn remove_capability(bits: u64, cap: Capability) -> u64 {
    bits & !cap.bit()
}

#[inline]
pub fn capability_count(bits: u64) -> usize {
    bits.count_ones() as usize
}
