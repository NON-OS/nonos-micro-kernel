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

use crate::protocol::{Request, E_INVAL, HDR_LEN, STATUS_LEN};
use crate::scsi;
use crate::server::respond;

// last_lba(4) + block_len(4) + block_count(8) + capacity_bytes(8), all
// little-endian for the local IPC caller. The derived count and byte size save
// every caller from repeating the off-by-one and the multiply.
const DECODED_LEN: usize = 24;

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let Some(cap) = scsi::parse_capacity(body) else {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    let base = HDR_LEN + STATUS_LEN;
    tx[base..base + 4].copy_from_slice(&cap.last_lba.to_le_bytes());
    tx[base + 4..base + 8].copy_from_slice(&cap.block_len.to_le_bytes());
    tx[base + 8..base + 16].copy_from_slice(&cap.block_count().to_le_bytes());
    tx[base + 16..base + 24].copy_from_slice(&cap.capacity_bytes().to_le_bytes());
    let _ = respond::payload(sender_pid, req, DECODED_LEN, tx);
}
