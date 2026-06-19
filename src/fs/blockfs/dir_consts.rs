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

pub(super) const REC_MAGIC: [u8; 8] = *b"NONOSDR1";
pub(super) const REC_COUNT_OFFSET: usize = 8;
pub(super) const REC_ENTRY_BASE: usize = 24;
pub(super) const ENTRY_BYTES: usize = 64;
pub(super) const NAME_BYTES: usize = 56;
pub(super) const MAX_ENTRIES: usize = 7;
