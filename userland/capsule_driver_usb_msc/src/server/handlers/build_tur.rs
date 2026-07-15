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

use crate::bot::{CommandBlockWrapper, CBW_FLAG_OUT};
use crate::protocol::{Request, CBW_LEN, HDR_LEN, STATUS_LEN};
use crate::scsi;
use crate::server::respond;
use crate::state::State;

pub fn handle(state: &mut State, sender_pid: u32, req: &Request, tx: &mut [u8]) {
    let (cdb, cdb_len) = scsi::test_unit_ready();
    let cbw = CommandBlockWrapper {
        tag: state.begin_command(0),
        data_len: 0,
        flags: CBW_FLAG_OUT,
        lun: 0,
        cdb_len,
        cdb,
    };
    cbw.write(&mut tx[HDR_LEN + STATUS_LEN..HDR_LEN + STATUS_LEN + CBW_LEN]);
    let _ = respond::payload(sender_pid, req, CBW_LEN, tx);
}
