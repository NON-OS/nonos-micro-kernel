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

pub(super) const IO_QID: u16 = 1;
pub(super) const IO_ENTRIES: u16 = 8;
pub(super) const SQ_BYTES: u64 = 4096;
pub(super) const CQ_BYTES: u64 = 4096;
pub(super) const PRP_LIST_BYTES: u64 = 4096;
pub const SECTOR_SIZE: usize = 512;
pub const MAX_SECTORS: u32 = 64;
pub(super) const DATA_BYTES: u64 = MAX_SECTORS as u64 * SECTOR_SIZE as u64;
pub(super) const PAGE: u64 = 4096;
pub(super) const COMPLETION_TIMEOUT_MS: u64 = 5_000;
