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

//! A directory as a chain of record blocks rather than one.
//!
//! A record block holds seven entries, which is what fits in 484 plaintext
//! bytes beside the header. A directory was one block, so a directory held
//! seven files and the eighth returned `DirectoryFull`. That is not a home
//! directory, and it is not something an application can work around.
//!
//! The header had twelve unused bytes after the count. Eight of them are now
//! the next record's address, and every volume written before this has zeros
//! there, which reads as no next record. So an old volume mounts and behaves
//! exactly as it did, and grows a second block the first time an eighth file
//! is created in one directory.

use alloc::vec::Vec;

use super::dir_block_header::{is_record, next};
use super::{BlockFsError, BlockFsNode};

/// A bound on the walk. Seven entries a block, so this is fifty thousand
/// files in one directory, and it stops a corrupt or circular next pointer
/// from hanging the kernel in a loop no timeout covers.
pub(super) const MAX_RECORDS: usize = 8192;

/// Every record block of `dir`, in order.
///
/// Each block's magic is checked as it is read, so a pointer into a data
/// block or into free space fails here rather than being read as entries.
pub(super) fn records(key: &[u8; 32], dir: &BlockFsNode) -> Result<Vec<u64>, BlockFsError> {
    let mut out = Vec::new();
    let mut lba = dir.first_record_lba;
    while lba != 0 {
        if out.len() >= MAX_RECORDS {
            return Err(BlockFsError::InvalidRecord);
        }
        let block = crate::fs::cryptoblock::read(key, lba).map_err(BlockFsError::CryptoBlock)?;
        if !is_record(&block) {
            return Err(BlockFsError::InvalidRecord);
        }
        out.push(lba);
        lba = next(&block);
    }
    Ok(out)
}
