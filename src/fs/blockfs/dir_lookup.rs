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

//! Finding a name in a directory.

use super::dir_block::{find, lba};
use super::dir_block_header::is_record;
use super::dir_chain::records;
use super::{BlockFsError, BlockFsNode};

/// The child's node address, or `None`.
///
/// Walks every record block of the directory. A name lives in at most one of
/// them, because `link` refuses a name that already resolves anywhere in the
/// chain.
pub fn lookup(key: &[u8; 32], dir: &BlockFsNode, name: &[u8]) -> Result<Option<u64>, BlockFsError> {
    for rec in records(key, dir)? {
        let block = crate::fs::cryptoblock::read(key, rec).map_err(BlockFsError::CryptoBlock)?;
        if !is_record(&block) {
            return Err(BlockFsError::InvalidRecord);
        }
        if let Some(i) = find(&block, name) {
            return Ok(Some(lba(&block, i)));
        }
    }
    Ok(None)
}
