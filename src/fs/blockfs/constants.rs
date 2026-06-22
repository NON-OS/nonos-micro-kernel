// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

pub(crate) const MAGIC: [u8; 8] = *b"NONOSFS1";
pub(crate) const NODE_MAGIC: [u8; 8] = *b"NONOSND1";
pub(crate) const VERSION: u64 = 1;
pub(crate) const HEADER_RING_SECTORS: u64 = 256;
pub(crate) const SUPERBLOCK_BYTES: usize = 104;
pub(crate) const NODE_DIGEST_OFFSET: usize = 96;
pub(crate) const NODE_BYTES: usize = 128;
pub(crate) const FIRST_ALLOC_LBA: u64 = HEADER_RING_SECTORS;
pub const MODE_DIR: u16 = 0x4000;
pub const MODE_FILE: u16 = 0x8000;
pub(crate) const MODE_755: u16 = 0o755;
