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

pub(super) const PAGE_SIZE: usize = 4096;
pub(super) const USER_SPACE_MAX: u64 = 0x0000_7FFF_FFFF_FFFF;
pub(super) const MAX_MMAP_SIZE: usize = 1 << 30;

pub(super) const PROT_WRITE: u32 = 0x2;
pub(super) const PROT_EXEC: u32 = 0x4;

#[inline]
pub(super) fn is_user_space(addr: u64, len: usize) -> bool {
    addr <= USER_SPACE_MAX && len <= (USER_SPACE_MAX - addr) as usize
}
