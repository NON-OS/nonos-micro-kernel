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

use super::deserialize::deserialize;
use super::{BlockFsError, BlockFsMount};

pub(super) fn read_header(key: &[u8; 32], lba: u64) -> Result<BlockFsMount, BlockFsError> {
    let payload = crate::fs::cryptoblock::read(key, lba).map_err(BlockFsError::CryptoBlock)?;
    let superblock = deserialize(&payload)?;
    Ok(BlockFsMount { header_lba: lba, superblock })
}
