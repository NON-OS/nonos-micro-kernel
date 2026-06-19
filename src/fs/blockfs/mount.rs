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

use super::constants::HEADER_RING_SECTORS;
use super::read_header::read_header;
use super::select_newer::select_newer;
use super::{BlockFsError, BlockFsMount};

pub fn mount(key: &[u8; 32]) -> Result<BlockFsMount, BlockFsError> {
    let mut best = None;
    for lba in 0..HEADER_RING_SECTORS {
        if let Ok(candidate) = read_header(key, lba) {
            best = select_newer(best, candidate);
        }
    }
    best.ok_or(BlockFsError::NotFormatted)
}
