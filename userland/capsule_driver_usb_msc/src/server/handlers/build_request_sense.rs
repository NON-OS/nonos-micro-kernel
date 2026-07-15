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

use crate::bot::{CommandBlockWrapper, CBW_FLAG_IN};
use crate::protocol::{Request, CBW_LEN, HDR_LEN, STATUS_LEN};
use crate::scsi;
use crate::server::respond;
use crate::state::State;

// The standard fixed-format sense buffer a device returns when none is
// requested explicitly.
const DEFAULT_SENSE_ALLOC: u8 = 18;

pub fn handle(state: &mut State, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let alloc_len = body.first().copied().unwrap_or(DEFAULT_SENSE_ALLOC);
    let (cdb, cdb_len) = scsi::request_sense(alloc_len);
    let cbw = CommandBlockWrapper {
        tag: state.begin_command(u32::from(alloc_len)),
        data_len: u32::from(alloc_len),
        flags: CBW_FLAG_IN,
        lun: 0,
        cdb_len,
        cdb,
    };
    cbw.write(&mut tx[HDR_LEN + STATUS_LEN..HDR_LEN + STATUS_LEN + CBW_LEN]);
    let _ = respond::payload(sender_pid, req, CBW_LEN, tx);
}
