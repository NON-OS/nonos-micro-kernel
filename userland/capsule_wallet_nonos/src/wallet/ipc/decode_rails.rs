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

use super::constants::HDR_LEN;
use crate::wallet::state::{Rail, MAX_RAILS};

pub fn decode_rails(buf: &[u8], rails: &mut [Rail; MAX_RAILS]) -> Result<usize, i32> {
    let count = u32::from_le_bytes([buf[8], buf[9], buf[10], buf[11]]) as usize;
    let mut off = HDR_LEN + 4;
    let mut n = 0;
    while n < count && n < MAX_RAILS {
        if off + 36 > buf.len() {
            return Err(-22);
        }
        let symbol_len = buf[off] as usize;
        if symbol_len > 8 || off + 36 + symbol_len > buf.len() {
            return Err(-22);
        }
        rails[n].symbol = [0; 8];
        rails[n].symbol[..symbol_len].copy_from_slice(&buf[off + 36..off + 36 + symbol_len]);
        rails[n].symbol_len = symbol_len as u8;
        rails[n].family = buf[off + 1];
        rails[n].status = u16::from_le_bytes([buf[off + 2], buf[off + 3]]);
        rails[n].flags =
            u32::from_le_bytes([buf[off + 4], buf[off + 5], buf[off + 6], buf[off + 7]]);
        rails[n].chain_id = u64::from_le_bytes([
            buf[off + 8],
            buf[off + 9],
            buf[off + 10],
            buf[off + 11],
            buf[off + 12],
            buf[off + 13],
            buf[off + 14],
            buf[off + 15],
        ]);
        rails[n].contract.copy_from_slice(&buf[off + 16..off + 36]);
        off += 36 + symbol_len;
        n += 1;
    }
    Ok(n)
}
