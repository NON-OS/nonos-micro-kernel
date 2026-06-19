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

use super::constants::{PLAIN_BLOCK_BYTES, SECTOR_BYTES};
use super::map_block::map_block_error;
use super::open::open;
use super::CryptoBlockError;

pub fn read(key: &[u8; 32], lba: u64) -> Result<[u8; PLAIN_BLOCK_BYTES], CryptoBlockError> {
    let mut sector = [0u8; SECTOR_BYTES];
    crate::hardware::block_device::read(lba, &mut sector).map_err(map_block_error)?;
    open(key, lba, &sector)
}
