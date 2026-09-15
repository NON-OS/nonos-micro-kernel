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

//! Reading the table of contents, which has to happen before anything else can
//! be walked.

use alloc::vec;
use alloc::vec::Vec;

use super::super::client::{capacity, read_blocks};
use super::super::error::BlkError;
use super::super::store::{sector_span, STORE_BASE_LBA};
use super::super::store_header::{entry_count, ENTRY_LEN, HEADER_LEN};
use super::super::store_toc::decode;
use super::super::wire::SECTOR_SIZE;
use super::types::Load;

impl Load {
    /// Read the header and the table of contents.
    ///
    /// Not budgeted: it is three block requests at most and nothing can be
    /// walked without it. The payloads are what needed breaking up.
    pub fn begin() -> Result<Self, BlkError> {
        let capacity_bytes =
            capacity()?.checked_mul(SECTOR_SIZE as u64).ok_or(BlkError::BadLength)?;
        let mut head = [0u8; SECTOR_SIZE];
        read_blocks(STORE_BASE_LBA, &mut head)?;
        let count = entry_count(&head)?;
        let mut toc = vec![0u8; sector_span(HEADER_LEN + ENTRY_LEN * count)];
        read_blocks(STORE_BASE_LBA, &mut toc)?;
        let toc = decode(&toc, count, capacity_bytes)?;
        Ok(Load { toc, idx: 0, data: Vec::new(), staged: Vec::with_capacity(count) })
    }
}
