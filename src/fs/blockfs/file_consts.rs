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

use crate::fs::cryptoblock::PLAIN_BLOCK_BYTES;

pub(super) const INDEX_MAGIC: [u8; 8] = *b"NONOSIX1";
pub(super) const INDEX_COUNT_OFFSET: usize = 8;
pub(super) const INDEX_PTR_BASE: usize = 16;
pub(super) const PTR_BYTES: usize = 8;
pub(super) const DATA_BYTES: usize = PLAIN_BLOCK_BYTES;
pub(super) const MAX_PTRS: usize = (PLAIN_BLOCK_BYTES - INDEX_PTR_BASE) / PTR_BYTES;
pub(crate) const MAX_FILE_BYTES: usize = MAX_PTRS * DATA_BYTES;
