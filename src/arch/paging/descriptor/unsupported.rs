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

//! An architecture whose descriptor format this kernel does not know.
//!
//! Every entry reads as absent and every entry built is invalid, so the shared
//! manager fails to map rather than writing a word that means something
//! unintended to hardware nobody has described here yet.

pub const ADDR_MASK: u64 = 0x0000_FFFF_FFFF_F000;

#[inline]
pub const fn leaf(_pa: u64, _flags: u64) -> u64 {
    0
}

#[inline]
pub const fn table(_pa: u64, _user_accessible: bool) -> u64 {
    0
}

#[inline]
pub const fn is_present(_entry: u64) -> bool {
    false
}

#[inline]
pub const fn is_block(_entry: u64) -> bool {
    false
}

#[inline]
pub const fn address(entry: u64) -> u64 {
    entry & ADDR_MASK
}

#[inline]
pub const fn is_writable(_entry: u64) -> bool {
    false
}

#[inline]
pub const fn is_user(_entry: u64) -> bool {
    false
}

#[inline]
pub const fn table_grants_user(_entry: u64) -> bool {
    false
}
