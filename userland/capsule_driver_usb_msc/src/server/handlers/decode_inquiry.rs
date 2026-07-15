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

// peripheral_type(1) + removable(1) + version(1) + vendor(8) + product(16) +
// revision(4).
const DECODED_LEN: usize = 31;

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let Some(data) = scsi::parse_inquiry(body) else {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    let base = HDR_LEN + STATUS_LEN;
    tx[base] = data.peripheral_type;
    tx[base + 1] = u8::from(data.removable);
    tx[base + 2] = data.version;
    tx[base + 3..base + 11].copy_from_slice(&data.vendor);
    tx[base + 11..base + 27].copy_from_slice(&data.product);
    tx[base + 27..base + 31].copy_from_slice(&data.revision);
    let _ = respond::payload(sender_pid, req, DECODED_LEN, tx);
}
