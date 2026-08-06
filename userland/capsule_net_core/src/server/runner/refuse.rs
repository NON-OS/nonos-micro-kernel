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

use crate::server::parse_req::HDR_LEN;
use crate::server::respond::reply;

/// Answer a request that could not be parsed.
///
/// Dropping it instead leaves the caller waiting on a reply that will never
/// come, so a malformed request costs it the full call timeout and reports
/// itself as a timeout rather than as the bad request it was. The header is
/// echoed back from the raw bytes because that is all that could be read.
pub fn refuse(sender_pid: u32, raw: &[u8], errno: u16, tx: &mut [u8]) {
    if raw.len() < HDR_LEN {
        // Too short to carry the fields a reply has to be addressed with.
        return;
    }
    let magic = u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]);
    let op = u16::from_le_bytes([raw[6], raw[7]]);
    let request_id = u32::from_le_bytes([raw[12], raw[13], raw[14], raw[15]]);
    let _ = reply(sender_pid, magic, op, errno, request_id, &[], tx);
}
