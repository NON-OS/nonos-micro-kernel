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

// sense_key(1) + asc(1) + ascq(1).
const DECODED_LEN: usize = 3;

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let Some(sense) = scsi::parse_sense(body) else {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    let base = HDR_LEN + STATUS_LEN;
    tx[base] = sense.sense_key;
    tx[base + 1] = sense.asc;
    tx[base + 2] = sense.ascq;
    let _ = respond::payload(sender_pid, req, DECODED_LEN, tx);
}
